use lux_camp_badge::led::{Animation, LedMatrix};

use rand::{rngs::SmallRng, Rng, SeedableRng};
use smart_leds::hsv::{hsv2rgb, Hsv};
use smart_leds_trait::{SmartLedsWrite, RGB8};
use std::time::Duration;

struct PerlinNoiseLight {
    seed: Vec<Vec<Vec<u8>>>,
    frame: usize,
}

impl PerlinNoiseLight {
    fn new(width: usize, height: usize) -> PerlinNoiseLight {
        let mut rnd = SmallRng::seed_from_u64(1);
        let mut seed = vec![vec![vec![127; 100]; height]; width];
        for x in 0..width {
            for y in 0..height {
                for t in 0..100 {
                    seed[x][y][t] = rnd.gen();
                }
            }
        }
        PerlinNoiseLight { seed, frame: 0 }
    }
}

pub struct PerlinAnimation(PerlinNoiseLight);

impl PerlinAnimation {
    pub fn build<Matrix, Driver>() -> Box<dyn Animation<Matrix> + Send>
    where
        Matrix: LedMatrix<Driver = Driver>,
        Driver: SmartLedsWrite<Color = RGB8>,
    {
        Box::new(Self(PerlinNoiseLight::new(Matrix::X, Matrix::Y)))
    }
}

impl<B, C: LedMatrix<Driver = B>> Animation<C> for PerlinAnimation
where
    B: SmartLedsWrite<Color = RGB8>,
{
    fn init(&mut self, _matrix: &mut C) -> Option<Duration> {
        Some(Duration::from_millis(250))
    }

    fn update(&mut self, _tick: Duration, matrix: &mut C) {
        let mut buf = Vec::with_capacity(<C as LedMatrix>::AREA);
        for _ in 0..<C as LedMatrix>::AREA {
            buf.push(hsv2rgb(Hsv {
                hue: self.0.seed[1][1][self.0.frame],
                sat: 255,
                val: 25,
            }));
        }
        let frame = self.0.frame;
        self.0.frame = (frame + 1) % 100;

        matrix.set_buf(&mut buf);
    }
}
