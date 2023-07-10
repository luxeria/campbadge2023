use crate::led::matrix::Animations::{Rainbow, RainbowSlide};
use esp_idf_svc::systime::EspSystemTime;
use smart_leds::hsv::{hsv2rgb, Hsv};
use smart_leds_trait::{SmartLedsWrite, RGB};
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use ws2812_esp32_rmt_driver::{Ws2812Esp32Rmt, RGB8};

use super::Animation;

use lazy_static::lazy_static;
lazy_static! {
    pub static ref STOP: Arc<Mutex<bool>> = Arc::new(Mutex::new(true));
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
    pixels: Vec<RGB<u8>>,
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

pub struct Dummy {}
impl Animation for Dummy {}
pub struct Matrix {
    animation: Box<dyn Animation + Send>,
    led_matrix: LedMatrix,
    //stop: Arc<Mutex<&'a mut bool>>,
}

impl Matrix {
    //pub fn new(stop: Arc<Mutex<&'a mut bool>>) -> Self {
    pub fn new() -> Self {
        let led_pin = 10;
        let led_channel = 0;
        Self {
            animation: Box::new(Dummy {}),
            led_matrix: LedMatrix::new(led_pin, led_channel, 5, 5),
            //stop,
        }
    }
    pub fn set(&mut self, mut animation: Box<dyn Animation + Send>) {
        if let Some(pixels) = animation.init() {
            self.draw(pixels);
        }
        self.animation = animation
    }

    fn draw(&self, pixels: Vec<RGB<u8>>) {}

    pub fn run(&mut self) {
        loop {
            if *STOP.lock().unwrap() {
                return;
            }
        }
    }
}
