#![feature(str_split_remainder)]

use embedded_svc::http::server::HandlerError;
use embedded_svc::http::Headers;
use embedded_svc::io::Read;
use embedded_svc::{http::Method, io::Write};
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::systime::EspSystemTime;
use esp_idf_svc::wifi::EspWifi;
use esp_idf_sys::{self as _}; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use lux_camp_badge::led::matrix::{self, Handle, Matrix};
use lux_camp_badge::led::{Animation, Color, LedMatrix};
use lux_camp_badge_animations::prelude::*;
use serde::Deserialize;
use smart_leds_trait::RGB8;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;
use ws2812_esp32_rmt_driver::Ws2812Esp32Rmt;

use esp_idf_svc::nvs::{EspDefaultNvsPartition, EspNvsPartition, NvsDefault};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    http::server::{Configuration, EspHttpServer},
};

/// Configuration of our LED matrix
const LED_PIN: u32 = 1;
const LED_CHANNEL: u8 = 0;
static INDEX_HTML: &str = include_str!("json_post_handler.html");

#[derive(Default)]
struct LuxBadge([Color<Self>; <Self as LedMatrix>::AREA]);

impl LedMatrix for LuxBadge {
    const X: usize = 5;
    const Y: usize = 5;
    type Driver = Ws2812Esp32Rmt;

    fn read_buf(&self) -> &[Color<Self>] {
        &self.0
    }

    fn set_buf(&mut self, buf: &mut [Color<Self>]) {
        if buf.len() == <Self as LedMatrix>::AREA {
            self.0.copy_from_slice(buf);
        }
    }

    fn set_2d(&mut self, x: usize, y: usize, color: &Color<Self>) {
        self.0[(y * <Self as LedMatrix>::Y) + (<Self as LedMatrix>::X - 1 - x)] = *color;
    }
}

/// The default static scene conveniently turns our LED matrix off.
type Off = Box<
    scene::Static<Color<LuxBadge>, { <LuxBadge as LedMatrix>::X }, { <LuxBadge as LedMatrix>::Y }>,
>;

#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

// Max payload length
const MAX_LEN: usize = 1024;

#[derive(Deserialize)]
struct FormData<'a> {
    color: &'a str,
    pixels: &'a str,
}

#[derive(Deserialize)]
struct FormDataAnimation<'a> {
    animation: &'a str,
}

#[derive(Copy, Clone)]
pub enum Animations {
    Rainbow,
    RainbowSlide,
}

type FrameBuf = [[Color<LuxBadge>; <LuxBadge as LedMatrix>::Y]; <LuxBadge as LedMatrix>::X];

impl<'a> From<&FormData<'a>> for FrameBuf {
    fn from(val: &FormData<'a>) -> Self {
        let mut buf = FrameBuf::default();
        let pixels = val.pixels.split(',').collect::<Vec<_>>();
        for y in 0..<LuxBadge as LedMatrix>::Y {
            for x in 0..<LuxBadge as LedMatrix>::X {
                buf[<LuxBadge as LedMatrix>::Y - 1 - y][x] = hex::decode(
                    pixels[y * <LuxBadge as LedMatrix>::Y + x]
                        .strip_prefix('#')
                        .unwrap(),
                )
                .map(|bytes| RGB8::new(bytes[0], bytes[1], bytes[2]))
                .unwrap()
            }
        }
        buf
    }
}

fn init() -> EspNvsPartition<NvsDefault> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    EspDefaultNvsPartition::take().unwrap()
}

fn connect_wifi(
    modem: impl Peripheral<P = esp_idf_hal::modem::Modem> + 'static,
) -> Box<EspWifi<'static>> {
    let sysloop = EspSystemEventLoop::take().unwrap();

    // The constant `CONFIG` is auto-generated by `toml_config`.
    let app_config = CONFIG;

    // Connect to the Wi-Fi network
    lux_camp_badge::wifi::connect(app_config.wifi_ssid, app_config.wifi_psk, modem, sysloop)
        .unwrap()
}

fn start_web_server(
    led_matrix: Arc<Mutex<Option<Handle<LuxBadge, Ws2812Esp32Rmt>>>>,
) -> EspHttpServer {
    let mut server = EspHttpServer::new(&Configuration::default()).unwrap();

    // http://<sta ip>/ handler
    server
        .fn_handler("/", Method::Get, |request| {
            let mut response = request.into_ok_response()?;
            response.write_all(INDEX_HTML.as_bytes())?;
            Ok(())
        })
        .unwrap();

    let h = Arc::clone(&led_matrix);
    server
        .fn_handler("/mode", Method::Get, move |req| {
            if req.uri().starts_with("/mode?set=off") {
                matrix::update(&h, Off::default())
                    .map_err(|_| HandlerError::new("matrix error"))?;
            }
            req.into_ok_response()?;
            Ok(())
        })
        .unwrap();

    let h = Arc::clone(&led_matrix);
    server
        .fn_handler("/animation", Method::Post, move |mut req| {
            let len = req.content_len().unwrap_or(0) as usize;

            if len > MAX_LEN {
                req.into_status_response(413)?
                    .write_all("Request too big".as_bytes())?;
                return Ok(());
            }

            let mut buf = vec![0; len];
            req.read_exact(&mut buf)?;
            let mut resp = req.into_ok_response()?;

            serde_json::from_slice::<FormDataAnimation>(&buf)
                .map(|form| {
                    let seed = EspSystemTime {}.now().as_millis() as u64;
                    let animation: Box<dyn Animation<LuxBadge> + Send + 'static> =
                        match form.animation {
                            "rainbow" => rainbow::Fade::build(1, None),
                            "rainbow-slide" => rainbow::Slide::build(5, None),
                            "flip" => random::Flip::build(seed),
                            "random" => random::P30::build(seed),
                            "perlin" => noise::PerlinAnimation::build(),
                            "gol" => gol::Gol::<
                                Color<LuxBadge>,
                                { <LuxBadge as LedMatrix>::X },
                                { <LuxBadge as LedMatrix>::Y },
                            >::build(
                                seed, 0.5, Some(128), Some(Duration::from_millis(250))
                            ),
                            _ => Off::default(),
                        };
                    matrix::update(&h, animation).unwrap();
                    write!(resp, "Displaying {}", form.animation)
                })
                .map_err(|_| resp.write_all("JSON error".as_bytes()))??;

            Ok(())
        })
        .unwrap();

    let h = Arc::clone(&led_matrix);
    server
        .fn_handler("/interactive", Method::Post, move |mut req| {
            let len = req.content_len().unwrap_or(0) as usize;

            if len > MAX_LEN {
                req.into_status_response(413)?
                    .write_all("Request too big".as_bytes())?;
                return Ok(());
            }

            let mut buf = vec![0; len];
            req.read_exact(&mut buf)?;
            let mut resp = req.into_ok_response()?;

            serde_json::from_slice::<FormData>(&buf)
                .map(|form| {
                    matrix::update(&h, Box::new(scene::Static((&form).into()))).unwrap();
                    write!(resp, "Interactive {:?} Color:{}", form.pixels, form.color)
                })
                .map_err(|_| resp.write_all("JSON error".as_bytes()))??;

            Ok(())
        })
        .unwrap();

    server
        .fn_handler("/brightness", Method::Get, move |request| {
            let level = match request.uri().split("?val=").nth(1) {
                Some("off") => None,
                Some(v) => v.parse::<u8>().map(|v| v as f32 / 100.0).ok(),
                _ => return Err(HandlerError::new("invalid brightness value")),
            };

            matrix::brightness(&led_matrix, level)
                .map_err(|_| HandlerError::new(r#"{ "error": "matrix failure" }"#))?;

            request
                .into_ok_response()?
                .write(format!("brightness: {}%", level.unwrap_or(0.5) * 200.0).as_bytes())?;
            Ok(())
        })
        .unwrap();

    println!("Server awaiting connection");

    server
}

fn main() -> ! {
    let _nvs = init();
    let peripherals = Peripherals::take().unwrap();
    let modem = peripherals.modem;

    let mut vcc_pin = PinDriver::output(peripherals.pins.gpio0).unwrap();
    vcc_pin.set_high().unwrap();

    // Setup HTTP server and LED matrix
    let led_matrix = Matrix::new(LuxBadge::default())
        .animation(random::P30::build(EspSystemTime {}.now().as_millis() as u64))
        .run(Ws2812Esp32Rmt::new(LED_CHANNEL, LED_PIN).unwrap())
        .unwrap();
    let _wifi = connect_wifi(modem);
    let _server = start_web_server(led_matrix);

    loop {
        sleep(Duration::from_secs(1));
    }
}
