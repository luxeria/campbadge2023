#![allow(clippy::type_complexity)] // This only applies to internal types.

/// Generic, performant, panic-free* and thread-safe implementation over an LED matrix:
///
/// * Correct usage should be guaranteed at compile time.
/// * Animations are abstracted via the [Animation] trait of this crate.
/// * LED backends (drivers) are abstracted via the `SmartLedsWrite` from the `smart_leds_trait` crate
///
/// \* Ultimately depends on the backend and animation implementations.
use esp_idf_svc::systime::EspSystemTime;
use lazy_static::lazy_static;
use smart_leds_trait::SmartLedsWrite;
use std::{
    fmt::Debug,
    marker::PhantomData,
    sync::{Arc, Mutex},
    thread::JoinHandle,
    time::Duration,
};

pub use self::state::AnimationSet;
use super::{Animation, MatrixConfig};

lazy_static! {
    static ref STOP: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
}

#[derive(Debug)]
pub enum Error<T: Debug> {
    /// A thread holding a mutex to an instance or `STOP` flag did panic.
    Poisoned,
    /// The thread running the matrix animations did panic.
    Paniced,
    /// The LED matrix driver backend returned an error while writing pixels.
    Driver(T),
}

/// Type states for the [MatrixBuilder].
mod state {
    /// Represents a builder state where the animation is set.
    pub struct AnimationSet;
}

/// Default backend that does nothing.
pub struct DummyBackend;
impl SmartLedsWrite for DummyBackend {
    type Error = ();
    type Color = ();

    fn write<T, I>(&mut self, _iterator: T) -> Result<(), Self::Error>
    where
        T: Iterator<Item = I>,
        I: Into<Self::Color>,
    {
        Ok(())
    }
}

/// A missing state in the builder means that a mandotory field was not yet set.
pub struct Missing<State>(PhantomData<fn() -> State>);
#[derive(Clone)]

/// Default matrix configuration without a meaningful config.
pub struct DummyConfig;
impl MatrixConfig for DummyConfig {
    const X: usize = 0;
    const Y: usize = 0;
    const AREA: usize = 0;
    type Backend = DummyBackend;
}
impl crate::led::Animation<DummyConfig> for () {}

/// Builder for [Matrix] where setting an Animation is mandatory.
/// Ensures that only valid instances of [Matrix] are created at compile time:
/// * Animation must be set
/// * Animation must be compatible with the targeted LED matrix backend
#[must_use]
pub struct MatrixBuilder<S: MatrixConfig, AnimationState> {
    animation: Box<dyn Animation<S> + Send>,
    fps: u8,
    marker: PhantomData<fn() -> AnimationState>,
}

impl<S: MatrixConfig, A> MatrixBuilder<S, A> {
    /// Set the `frames per seconds` (FPS) rate of the animations.
    /// In other words, define the refresh rate of the LED matrix.
    pub fn fps(mut self, n: u8) -> Self {
        self.fps = n;
        self
    }
}

impl<S: MatrixConfig> MatrixBuilder<S, Missing<AnimationSet>> {
    /// Set the initial animation to be displayed.
    ///
    /// You can later change the animation using the [update] function of this module.
    pub fn animation<Config>(
        self,
        animation: Box<dyn Animation<Config> + Send>,
    ) -> MatrixBuilder<Config, AnimationSet>
    where
        Config: MatrixConfig,
    {
        MatrixBuilder {
            animation,
            fps: self.fps,
            marker: PhantomData,
        }
    }
}

impl<S, B> MatrixBuilder<S, AnimationSet>
where
    S: MatrixConfig<Backend = B>,
    B: SmartLedsWrite + Send + 'static,
    B::Error: Send + Debug,
{
    /// Start the matrix in a background thread.
    pub fn run(
        self,
        backend: B,
    ) -> Result<Arc<Mutex<Option<Handle<S, B>>>>, Error<<B as SmartLedsWrite>::Error>> {
        let mut matrix = Matrix {
            animation: self.animation,
            backend,
            frame_time: Duration::from_millis(1000 / self.fps as u64),
            tick: EspSystemTime {}.now(),
        };
        matrix.init_animation()?;
        Ok(Arc::new(Mutex::new(Some(Handle(matrix.run())))))
    }
}

/// A generic LED matrix implementation.
///
/// Create an instance via the [MatrixBuilder] and change animations
/// at runtime via the [update] function using the [Handle] from the builder.
/// This guarantees correct and thread-safe usage.
pub struct Matrix<S, B>
where
    S: MatrixConfig<Backend = B>,
    B: SmartLedsWrite + Send,
    B::Error: Send,
{
    animation: Box<dyn Animation<S> + Send>,
    backend: B,
    frame_time: Duration,
    tick: Duration,
}

impl<S, B> Matrix<S, B>
where
    S: MatrixConfig<Backend = B>,
    B: SmartLedsWrite + Send + 'static,
    B::Error: Send + Debug,
{
    /// Creates a new [MatrixBuilder] with the following defaults:
    /// * `fps`: 24
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> MatrixBuilder<DummyConfig, Missing<AnimationSet>> {
        MatrixBuilder {
            animation: Box::new(()),
            fps: 24,
            marker: PhantomData,
        }
    }

    fn init_animation(&mut self) -> Result<(), Error<<B as SmartLedsWrite>::Error>> {
        if let Some(pixels) = self.animation.init() {
            self.draw(pixels)?
        }
        Ok(())
    }

    fn set_animation(
        &mut self,
        animation: Box<dyn Animation<S> + Send>,
    ) -> Result<(), Error<<B as SmartLedsWrite>::Error>> {
        self.animation = animation;
        self.init_animation()
    }

    fn draw<I>(&mut self, pixels: I) -> Result<(), Error<<B as SmartLedsWrite>::Error>>
    where
        I: IntoIterator<Item = <B as SmartLedsWrite>::Color>,
    {
        self.backend
            .write(pixels.into_iter())
            .map_err(Error::Driver)
    }

    fn run(mut self) -> JoinHandle<Result<Matrix<S, B>, Error<<B as SmartLedsWrite>::Error>>> {
        std::thread::spawn(|| loop {
            self.tick = EspSystemTime {}.now();

            if *STOP.lock().map_err(|_| Error::Poisoned)? {
                return Ok(self);
            }

            if let Some(pixels) = self.animation.update(self.tick) {
                self.draw(pixels)?;
            }

            std::thread::sleep(self.frame_time - (EspSystemTime {}.now() - self.tick));
        })
    }
}

/// Wrapper type for the `JoinHandle` of the thread in which the matrix is running.
/// This allows for thread-safe sharing of the handle.
pub struct Handle<S, B>(JoinHandle<Result<Matrix<S, B>, Error<<B as SmartLedsWrite>::Error>>>)
where
    S: MatrixConfig<Backend = B>,
    B: SmartLedsWrite + Send,
    B::Error: Send + Debug;

impl<S, B> Handle<S, B>
where
    S: MatrixConfig<Backend = B>,
    B: SmartLedsWrite + Send + 'static,
    B::Error: Send + Debug,
{
    fn start(mut matrix: Matrix<S, B>) -> Result<Self, Error<<B as SmartLedsWrite>::Error>> {
        *STOP.lock().map_err(|_| Error::Poisoned)? = false;
        matrix.init_animation()?;
        Ok(Self(matrix.run()))
    }

    /// Restart with a new animation (if there is already a running instance).
    fn update(
        self,
        animation: Box<dyn Animation<S> + Send>,
    ) -> Result<Self, Error<<B as SmartLedsWrite>::Error>> {
        let mut matrix = self.stop().map_err(|_| Error::Poisoned)?;
        matrix.set_animation(animation)?;
        Self::start(matrix)
    }

    /// Stop the matrix and return the underlying instance if it was running.
    fn stop(self) -> Result<Matrix<S, B>, Error<<B as SmartLedsWrite>::Error>> {
        *STOP.lock().map_err(|_| Error::Poisoned)? = true;
        self.0.join().map_err(|_| Error::Paniced)?
    }
}

pub fn update<S, B>(
    handle: &Arc<Mutex<Option<Handle<S, B>>>>,
    animation: Box<dyn Animation<S> + Send>,
) -> Result<(), Error<<B as SmartLedsWrite>::Error>>
where
    S: MatrixConfig<Backend = B>,
    B: SmartLedsWrite + Send + 'static,
    B::Error: Send + Debug,
{
    let mut handle = handle.lock().map_err(|_| Error::Poisoned)?;
    if let Some(inner) = handle.take() {
        let _ = handle.insert(inner.update(animation)?);
    }
    Ok(())
}
