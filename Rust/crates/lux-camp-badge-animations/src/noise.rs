use lux_camp_badge::led::{Animation, LedMatrix};

use glm;
use smart_leds::hsv::{hsv2rgb, Hsv};
use smart_leds_trait::{SmartLedsWrite, RGB8};
use std::time::Duration;

struct PerlinNoiseLight {
    frame: f32,
    scale_factor: f32,
}

impl PerlinNoiseLight {
    fn new(scale_factor: f32) -> PerlinNoiseLight {
        PerlinNoiseLight {
            frame: 0.,
            scale_factor,
        }
    }
}

pub struct PerlinAnimation(PerlinNoiseLight);

impl PerlinAnimation {
    pub fn build<Matrix, Driver>() -> Box<dyn Animation<Matrix> + Send>
    where
        Matrix: LedMatrix<Driver = Driver>,
        Driver: SmartLedsWrite<Color = RGB8>,
    {
        Box::new(Self(PerlinNoiseLight::new(0.05)))
    }
}

impl<B, C: LedMatrix<Driver = B>> Animation<C> for PerlinAnimation
where
    B: SmartLedsWrite<Color = RGB8>,
{
    fn init(&mut self, _matrix: &mut C) -> Option<Duration> {
        Some(Duration::from_millis(100))
    }

    fn update(&mut self, _tick: Duration, matrix: &mut C) {
        let mut buf = Vec::with_capacity(<C as LedMatrix>::AREA);
        let frame = self.0.frame;
        for x in 0..<C as LedMatrix>::X {
            for y in 0..<C as LedMatrix>::Y {
                let hue = glm::builtin::noise1(glm::vec3(
                    x as f32 * self.0.scale_factor,
                    y as f32 * self.0.scale_factor,
                    frame,
                ));
                let hue = ((hue * 255.0) + 127.0) as u8;
                buf.push(hsv2rgb(Hsv {
                    hue,
                    sat: 255,
                    val: 255,
                }));
            }
        }
        self.0.frame = frame + 0.05;

        matrix.set_buf(&mut buf);
    }
}
