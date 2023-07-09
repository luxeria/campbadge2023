#![feature(str_split_remainder)]

mod led_matrix;
mod rgb2hsv;

use std::num::ParseFloatError;
use std::ops::DerefMut;
use std::rc::Rc;
use std::str::{FromStr, Split};


use std::sync::{Arc, Condvar, Mutex};
use crate::led_matrix::{LedMatrix, LedState};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use log::*;
use rgb2hsv::{hsv2rgb, rgb2hsv};
use smart_leds::RGB8;
use std::thread::sleep;
use std::time::Duration;
use esp_idf_hal::peripherals::Peripherals;
use anyhow::{bail, Result};

use embedded_svc::{http::Method, io::Write};
use embedded_svc::http::Headers;
use embedded_svc::io::Read;

use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
};
use esp_idf_svc::nvs::EspDefaultNvsPartition;

use ws2812_esp32_rmt_driver::driver::color;
use crate::led_matrix::Animations::{Rainbow, RainbowSlide};


fn main() -> ! {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    let nvs = EspDefaultNvsPartition::take().unwrap();


    let led_pin = 10;
    let led_channel = 0;
    let mut leds = LedMatrix::new(led_pin, led_channel, 5, 5);

    let mut led_state = LedState::new();

    leds.set_all_pixel( RGB8::new(25, 0, 0));
    leds.write_pixels();
    info!("Hello, world!");

    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take().unwrap();


    leds.set_all_pixel( RGB8::new(0, 25, 25));
    leds.write_pixels();


    loop {

        led_state = led_state.tick(&mut leds);
        sleep(Duration::from_millis(1));

    }
}
