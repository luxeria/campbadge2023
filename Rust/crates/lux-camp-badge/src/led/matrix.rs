use crate::led::matrix::Animations::{Rainbow, RainbowSlide};
use esp_idf_svc::systime::EspSystemTime;
use lazy_static::lazy_static;
use smart_leds::hsv::{hsv2rgb, Hsv};
use smart_leds_trait::{SmartLedsWrite, RGB};
use std::{
    marker::PhantomData,
    sync::{Arc, Mutex},
    thread::JoinHandle,
    time::Duration,
};
use ws2812_esp32_rmt_driver::{Ws2812Esp32Rmt, RGB8};

pub use self::state::AnimationSet;

use super::{Animation, FrameBuf, MatrixConfig};

lazy_static! {
    static ref STOP: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
}

#[derive(Copy, Clone)]
pub enum Animations {
    Rainbow,
    RainbowSlide,
}
#[derive(Copy, Clone)]
pub enum LedState {
    Animation {
        animation: Animations,
        frame: u16,
        last_tick: Duration,
    },
    Interactive,
    Off,
}

impl Default for LedState {
    fn default() -> Self {
        Self::Animation {
            animation: RainbowSlide,
            frame: 0,
            last_tick: EspSystemTime {}.now(),
        }
    }
}

impl LedState {
    pub fn set_animation(self, animation: Animations) -> Self {
        LedState::Animation {
            animation,
            frame: 0,
            last_tick: EspSystemTime {}.now(),
        }
    }
    pub fn set_off(self) -> Self {
        LedState::Off
    }
    pub fn set_interactive(self) -> Self {
        LedState::Interactive
    }

    pub fn tick(self, led_matrix: &mut LedMatrix) -> Self {
        match self {
            LedState::Animation {
                animation,
                frame,
                last_tick,
            } => match animation {
                Rainbow => Self::rainbow(led_matrix, animation, frame, last_tick),
                RainbowSlide => Self::rainbow_slide(led_matrix, animation, frame, last_tick),
            },
            LedState::Interactive => self,
            LedState::Off => self,
        }
    }

    fn rainbow_slide(
        led_matrix: &mut LedMatrix,
        animation: Animations,
        frame: u16,
        last_tick: Duration,
    ) -> LedState {
        if (EspSystemTime {}.now() - last_tick) > Duration::from_millis(100) {
            //led_matrix.set_all_pixel(hsv2rgb(Hsv { hue: (frame % 255) as u8, sat: 255, val: 25 }));
            led_matrix
                .pixels
                .iter_mut()
                .enumerate()
                .for_each(|(i, pixel)| {
                    *pixel = hsv2rgb(Hsv {
                        hue: ((frame + (i * 5) as u16) % 255) as u8,
                        sat: 255,
                        val: 25,
                    })
                });
            led_matrix.write_pixels();
            LedState::Animation {
                animation,
                frame: (frame + 5) % 255,
                last_tick: EspSystemTime {}.now(),
            }
        } else {
            LedState::Animation {
                animation,
                frame,
                last_tick,
            }
        }
    }

    fn rainbow(
        led_matrix: &mut LedMatrix,
        animation: Animations,
        frame: u16,
        last_tick: Duration,
    ) -> LedState {
        if (EspSystemTime {}.now() - last_tick) > Duration::from_millis(100) {
            led_matrix.set_all_pixel(hsv2rgb(Hsv {
                hue: (frame % 255) as u8,
                sat: 255,
                val: 25,
            }));
            led_matrix.write_pixels();
            LedState::Animation {
                animation,
                frame: (frame + 5) % 255,
                last_tick: EspSystemTime {}.now(),
            }
        } else {
            LedState::Animation {
                animation,
                frame,
                last_tick,
            }
        }
    }
}

pub struct LedMatrix {
    led_rows: usize,
    led_columns: usize,
    pixels: FrameBuf,
    ws2812: Ws2812Esp32Rmt,
    _state: LedState,
}

impl LedMatrix {
    pub fn new(led_pin: u32, led_channel: u8, led_rows: usize, led_columns: usize) -> Self {
        let pixels = vec![RGB8::new(0, 0, 0); (led_rows * led_columns) as usize];
        LedMatrix {
            led_rows,
            led_columns,
            pixels,
            ws2812: Ws2812Esp32Rmt::new(led_channel, led_pin).unwrap(),
            _state: LedState::default(),
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: RGB8) {
        self.pixels[x + y * self.led_columns] = color;
    }
    pub fn set_all_pixel(&mut self, color: RGB8) {
        self.pixels.iter_mut().for_each(|pixel| *pixel = color);
    }
    pub fn get_pixel(&mut self, x: usize, y: usize) -> RGB8 {
        self.pixels[x + y * self.led_columns]
    }
    pub fn write_pixels(&mut self) {
        let pixels = self.pixels.iter().copied();
        self.ws2812.write(pixels).unwrap();
    }
    pub fn led_rows(&self) -> u8 {
        self.led_rows as u8
    }
    pub fn led_columns(&self) -> u8 {
        self.led_columns as u8
    }
}

/// Type states for the [MatrixBuilder].
mod state {
    /// Represents a builder state where the animation is set.
    pub struct AnimationSet;
}

pub struct Missing<State>(PhantomData<fn() -> State>);
#[derive(Clone)]
pub struct DummyConfig;
impl MatrixConfig for DummyConfig {
    const X: usize = 0;
    const Y: usize = 0;
    const AREA: usize = 0;
}
impl crate::led::Animation<DummyConfig> for () {}

/// Builder for [Matrix] where setting an Animation is mandatory.
#[must_use]
pub struct MatrixBuilder<S: MatrixConfig, AnimationState> {
    animation: Box<dyn Animation<S> + Send>,
    led_pin: u32,
    led_channel: u8,
    fps: u8,
    marker: PhantomData<fn() -> AnimationState>,
}

impl<S: MatrixConfig, A> MatrixBuilder<S, A> {
    pub fn led_pin(mut self, no: u32) -> Self {
        self.led_pin = no;
        self
    }

    pub fn led_channel(mut self, no: u8) -> Self {
        self.led_channel = no;
        self
    }

    pub fn fps(mut self, n: u8) -> Self {
        self.fps = n;
        self
    }
}

impl<S: MatrixConfig> MatrixBuilder<S, Missing<AnimationSet>> {
    pub fn animation<Config>(
        self,
        animation: Box<dyn Animation<Config> + Send>,
    ) -> MatrixBuilder<Config, AnimationSet>
    where
        Config: MatrixConfig,
    {
        MatrixBuilder {
            animation,
            led_channel: self.led_channel,
            led_pin: self.led_pin,
            fps: self.fps,
            marker: PhantomData,
        }
    }
}

impl<S: MatrixConfig> MatrixBuilder<S, AnimationSet> {
    /// Start the matrix in another thread.
    ///
    /// If there is already another instance running, the other instance will be stopped.
    pub fn run(self) -> Arc<Mutex<Option<Handle<S>>>> {
        let led_matrix = LedMatrix::new(
            self.led_pin,
            self.led_channel,
            <S as MatrixConfig>::X,
            <S as MatrixConfig>::Y,
        );

        let mut matrix = Matrix {
            animation: self.animation,
            led_matrix,
            frame_time: Duration::from_millis(1000 / self.fps as u64),
            tick: EspSystemTime {}.now(),
        };
        matrix.init_animation();
        Arc::new(Mutex::new(Some(Handle(matrix.run()))))
    }
}

pub struct Matrix<S: MatrixConfig> {
    animation: Box<dyn Animation<S> + Send>,
    led_matrix: LedMatrix,
    frame_time: Duration,
    tick: Duration,
}

impl<S: MatrixConfig> Matrix<S> {
    /// Creates a new [MatrixBuilder] with the following defaults:
    /// * `led_pin`: 10
    /// * `led_channel`: 0
    /// * `fps`: 24
    pub fn new() -> MatrixBuilder<DummyConfig, Missing<AnimationSet>> {
        MatrixBuilder {
            animation: Box::new(()),
            led_pin: 10,
            led_channel: 0,
            fps: 24,
            marker: PhantomData,
        }
    }

    fn init_animation(&mut self) {
        if let Some(pixels) = self.animation.init() {
            self.draw(&pixels);
        }
    }

    fn set_animation(&mut self, animation: Box<dyn Animation<S> + Send>) {
        self.animation = animation;
        self.init_animation();
    }

    fn draw(&mut self, pixels: &FrameBuf) {
        for x in 0..self.led_matrix.led_columns {
            for y in 0..self.led_matrix.led_rows {
                let idx = (x * self.led_matrix.led_columns) + y;
                self.led_matrix.set_pixel(x, y, pixels[idx as usize])
            }
        }
    }

    fn run(mut self) -> JoinHandle<Matrix<S>> {
        std::thread::spawn(|| loop {
            self.tick = EspSystemTime {}.now();

            if *STOP.lock().unwrap() {
                return self;
            }

            if let Some(pixels) = self.animation.update(self.tick) {
                self.draw(&pixels);
                self.led_matrix.write_pixels();
            }

            std::thread::sleep(self.frame_time - (EspSystemTime {}.now() - self.tick));
        })
    }
}

pub struct Handle<S: MatrixConfig>(JoinHandle<Matrix<S>>);

impl<S: MatrixConfig> Handle<S> {
    fn start(mut matrix: Matrix<S>) -> Self {
        *STOP.lock().unwrap() = false;
        matrix.init_animation();
        Self(matrix.run())
    }

    /// Restart with a new animation (if there is already a running instance).
    fn update(self, animation: Box<dyn Animation<S> + Send>) -> Self {
        let mut matrix = self.stop();
        matrix.set_animation(animation);
        Self::start(matrix)
    }

    /// Stop the matrix and return the underlying instance if it was running.
    fn stop(self) -> Matrix<S> {
        *STOP.lock().unwrap() = true;
        self.0.join().unwrap()
    }
}

pub fn update<S>(handle: &Arc<Mutex<Option<Handle<S>>>>, animation: Box<dyn Animation<S> + Send>)
where
    S: MatrixConfig,
{
    let mut handle = handle.lock().unwrap();
    if let Some(inner) = handle.take() {
        let _ = handle.insert(inner.update(animation));
    }
}
