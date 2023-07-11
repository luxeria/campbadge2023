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

use super::{Animation, FrameBuf, MatrixSize};

pub struct Config;
impl MatrixSize for Config {
    const X: usize = 5;
    const Y: usize = 5;
}

lazy_static! {
    static ref STOP: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    static ref INSTANCE: Arc<Mutex<Option<JoinHandle<Matrix<Config>>>>> =
        Arc::new(Mutex::new(None));
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
    led_rows: u8,
    led_columns: u8,
    pixels: FrameBuf,
    ws2812: Ws2812Esp32Rmt,
    _state: LedState,
}

impl LedMatrix {
    pub fn new(led_pin: u32, led_channel: u8, led_rows: u8, led_columns: u8) -> Self {
        let pixels = vec![RGB8::new(0, 0, 0); (led_rows * led_columns) as usize];
        LedMatrix {
            led_rows,
            led_columns,
            pixels,
            ws2812: Ws2812Esp32Rmt::new(led_channel, led_pin).unwrap(),
            _state: LedState::default(),
        }
    }

    pub fn set_pixel(&mut self, x: u8, y: u8, color: RGB8) {
        self.pixels[(x + y * self.led_columns) as usize] = color;
    }
    pub fn set_all_pixel(&mut self, color: RGB8) {
        self.pixels.iter_mut().for_each(|pixel| *pixel = color);
    }
    pub fn get_pixel(&mut self, x: u8, y: u8) -> RGB8 {
        self.pixels[(x + y * self.led_columns) as usize]
    }
    pub fn write_pixels(&mut self) {
        let pixels = self.pixels.iter().copied();
        self.ws2812.write(pixels).unwrap();
    }
    pub fn led_rows(&self) -> u8 {
        self.led_rows
    }
    pub fn led_columns(&self) -> u8 {
        self.led_columns
    }
}

/// Type states for the [MatrixBuilder].
mod state {
    /// Represents a builder state where the animation is set.
    pub struct AnimationSet;
}

pub struct Missing<State>(PhantomData<fn() -> State>);
impl crate::led::Animation for () {
    type Dimension = Config;
}

/// Builder for [Matrix] where setting an Animation is mandatory.
#[must_use]
pub struct MatrixBuilder<Size: MatrixSize, AnimationState> {
    animation: Box<dyn Animation<Dimension = Size> + Send>,
    led_pin: u32,
    led_channel: u8,
    dimension_x: u8,
    dimension_y: u8,
    fps: u8,
    marker: PhantomData<fn() -> AnimationState>,
}

impl<S: MatrixSize, A> MatrixBuilder<S, A> {
    pub fn led_pin(mut self, no: u32) -> Self {
        self.led_pin = no;
        self
    }

    pub fn led_channel(mut self, no: u8) -> Self {
        self.led_channel = no;
        self
    }

    pub fn dimension_x(mut self, n: u8) -> Self {
        self.dimension_x = n;
        self
    }

    pub fn dimension_y(mut self, n: u8) -> Self {
        self.dimension_y = n;
        self
    }

    pub fn fps(mut self, n: u8) -> Self {
        self.fps = n;
        self
    }
}

impl<S: MatrixSize> MatrixBuilder<S, Missing<AnimationSet>> {
    pub fn animation(
        self,
        animation: Box<dyn Animation<Dimension = S> + Send>,
    ) -> MatrixBuilder<S, AnimationSet> {
        MatrixBuilder {
            animation,
            led_channel: self.led_channel,
            led_pin: self.led_pin,
            dimension_x: self.dimension_x,
            dimension_y: self.dimension_y,
            fps: 24,
            marker: PhantomData,
        }
    }
}

impl MatrixBuilder<Config, AnimationSet> {
    /// Start the matrix in another thread.
    ///
    /// If there is already another instance running, the other instance will be stopped.
    pub fn run(self) {
        stop();

        let led_matrix = LedMatrix::new(
            self.led_pin,
            self.led_channel,
            self.dimension_x,
            self.dimension_y,
        );

        start(Matrix {
            animation: self.animation,
            led_matrix,
            frame_time: Duration::from_millis(1000 / self.fps as u64),
            tick: EspSystemTime {}.now(),
        })
    }
}

pub struct Matrix<S: MatrixSize> {
    animation: Box<dyn Animation<Dimension = S> + Send>,
    led_matrix: LedMatrix,
    frame_time: Duration,
    tick: Duration,
}

impl Matrix<Config> {
    /// Creates a new [MatrixBuilder] with the following defaults:
    /// * `led_pin`: 10
    /// * `led_channel`: 0
    /// * `dimension_x`: 5
    /// * `dimension_y`: 5
    /// * `fps`: 24
    pub fn new() -> MatrixBuilder<Config, Missing<AnimationSet>> {
        MatrixBuilder {
            animation: Box::new(()),
            led_pin: 10,
            led_channel: 0,
            dimension_x: 5,
            dimension_y: 5,
            fps: 24,
            marker: PhantomData,
        }
    }

    fn init_animation(&mut self) {
        if let Some(pixels) = self.animation.init() {
            self.draw(&pixels);
        }
    }

    fn set_animation(&mut self, animation: Box<dyn Animation<Dimension = Config> + Send>) {
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

    fn run(mut self) -> JoinHandle<Self> {
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

fn start(mut matrix: Matrix<Config>) {
    matrix.init_animation();
    *INSTANCE.lock().unwrap() = Some(matrix.run());
}

/// Restart with a new animation (if there is already a running instance).
pub fn update(animation: Box<dyn Animation<Dimension = Config> + Send>) {
    if let Some(mut matrix) = stop() {
        matrix.set_animation(animation);
        start(matrix);
    };
}

/// Stop the matrix and return the underlying instance if it was running.
pub fn stop() -> Option<Matrix<Config>> {
    *STOP.lock().unwrap() = true;
    let mut lock = INSTANCE.lock().unwrap();
    let matrix = lock.take().map(|handle| handle.join().unwrap());
    *STOP.lock().unwrap() = false;
    matrix
}
