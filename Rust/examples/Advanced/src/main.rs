#![feature(str_split_remainder)]

use embedded_svc::http::Headers;
use embedded_svc::io::Read;
use embedded_svc::{http::Method, io::Write};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::wifi::EspWifi;
use esp_idf_sys::{self as _}; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use lux_camp_badge::led::matrix::{self, Handle, Matrix};
use lux_camp_badge::led::{Animation, LedMatrix};
//use lux_camp_badge_animations::rainbow::{FadingRainbow, SlidingRainbow};
//use lux_camp_badge_animations::random::RandomAnimation;
use lux_camp_badge_animations::static_scene::Scene;
use serde::Deserialize;
use smart_leds::RGB8;
use smart_leds_trait::SmartLedsWrite;
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
#[derive(Default)]
struct LuxBadge(
    [<<Self as LedMatrix>::Backend as SmartLedsWrite>::Color; <Self as LedMatrix>::AREA],
);
impl LedMatrix for LuxBadge {
    const AREA: usize = 25;
    const X: usize = 5;
    const Y: usize = 5;
    const Z: usize = 0;
    type Backend = Ws2812Esp32Rmt;

    fn read(&self) -> &[<Self::Backend as SmartLedsWrite>::Color] {
        &self.0
    }
}

const LED_PIN: u32 = 10;
const LED_CHANNEL: u8 = 0;
static INDEX_HTML: &str = include_str!("json_post_handler.html");

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
struct FormDataMode<'a> {
    mode: &'a str,
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

fn init() -> EspNvsPartition<NvsDefault> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    EspDefaultNvsPartition::take().unwrap()
}

fn connect_wifi() -> Box<EspWifi<'static>> {
    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take().unwrap();

    // The constant `CONFIG` is auto-generated by `toml_config`.
    let app_config = CONFIG;

    // Connect to the Wi-Fi network
    lux_camp_badge::wifi::connect(
        app_config.wifi_ssid,
        app_config.wifi_psk,
        peripherals.modem,
        sysloop,
    )
    .unwrap()
}

fn start_web_server(
    led_matrix: Arc<Mutex<Option<Handle<LuxBadge, Ws2812Esp32Rmt>>>>,
) -> EspHttpServer {
    let mut server = EspHttpServer::new(&Configuration::default()).unwrap();

    // http://<sta ip>/ handler
    server
        .fn_handler("/", Method::Get, |request| {
            let mut response = request.into_ok_response().unwrap();
            response.write_all(INDEX_HTML.as_bytes())?;
            Ok(())
        })
        .unwrap();

    let h = Arc::clone(&led_matrix);
    server
        .fn_handler("/mode", Method::Post, move |mut req| {
            let len = req.content_len().unwrap_or(0) as usize;

            if len > MAX_LEN {
                req.into_status_response(413)?
                    .write_all("Request too big".as_bytes())?;
                return Ok(());
            }

            let mut buf = vec![0; len];
            req.read_exact(&mut buf)?;
            let mut resp = req.into_ok_response()?;

            serde_json::from_slice::<FormDataMode>(&buf)
                .map(|form| {
                    let animation: Box<dyn Animation<LuxBadge> + Send + 'static> = match form.mode {
                        //"animation" => Box::new(SlidingRainbow::new(4, None)),
                        "interactive" | "off" => Box::new(Scene(vec![
                            RGB8::new(0, 0, 0);
                            <LuxBadge as LedMatrix>::AREA
                        ])),
                        _ => return Ok(()),
                    };
                    matrix::update(&h, animation).unwrap();
                    write!(resp, "Hello, {}", form.mode)
                })
                .map_err(|_| resp.write_all("JSON error".as_bytes()))??;

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
                    let animation: Box<dyn Animation<LuxBadge> + Send + 'static> =
                        match form.animation {
                            //"rainbow" => Box::new(FadingRainbow::new(1, None)),
                            //"rainbow-slide" => Box::new(SlidingRainbow::new(5, None)),
                            //"random" => Box::<RandomAnimation>::default(),
                            _ => Box::new(Scene(vec![
                                RGB8::new(0, 0, 0);
                                <LuxBadge as LedMatrix>::AREA
                            ])),
                        };
                    matrix::update(&h, animation).unwrap();
                    write!(resp, "Displaying {}", form.animation)
                })
                .map_err(|_| resp.write_all("JSON error".as_bytes()))??;

            Ok(())
        })
        .unwrap();

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
                    let pixels = form
                        .pixels
                        .split(',')
                        .map(|pixel| {
                            hex::decode(pixel.strip_prefix('#').unwrap())
                                .map(|bytes| RGB8::new(bytes[0], bytes[1], bytes[2]))
                                .unwrap()
                        })
                        .collect();
                    matrix::update(&led_matrix, Box::new(Scene(pixels))).unwrap();
                    write!(resp, "Hello, {} Color:{}", form.pixels, form.color)
                })
                .map_err(|_| resp.write_all("JSON error".as_bytes()))??;

            Ok(())
        })
        .unwrap();

    println!("Server awaiting connection");

    server
}

fn main() -> ! {
    let _nvs = init();
    let _wifi = connect_wifi();

    // Setup HTTP server and LED matrix
    let led_matrix = Matrix::<_, _>::new(LuxBadge::default())
        //.animation::<LuxBadge>(Box::<RandomAnimation>::default())
        .animation(Box::<Scene>::default())
        .run(Ws2812Esp32Rmt::new(LED_CHANNEL, LED_PIN).unwrap())
        .unwrap();
    let _server = start_web_server(led_matrix);

    loop {
        sleep(Duration::from_secs(1));
    }
}
