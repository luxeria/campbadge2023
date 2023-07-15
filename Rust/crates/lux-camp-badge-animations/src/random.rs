use lux_camp_badge::led::{Animation, LedMatrix};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use smart_leds_trait::{SmartLedsWrite, RGB8};
use std::time::Duration;

/// Every 250ms, each pixel has a 30% chance of getting colored randomly.
pub struct Random {
    last_tick: Duration,
    rng: SmallRng,
}

impl Random {
    pub fn boxed<Matrix, Driver>(seed: u64) -> Box<dyn Animation<Matrix> + Send>
    where
        Matrix: LedMatrix<Driver = Driver>,
        Driver: SmartLedsWrite<Color = RGB8>,
    {
        Box::new(Self {
            last_tick: Duration::ZERO,
            rng: SmallRng::seed_from_u64(seed),
        })
    }
}

impl<B, C: LedMatrix<Driver = B>> Animation<C> for Random
where
    B: SmartLedsWrite<Color = RGB8>,
{
    fn init(&mut self, _matrix: &mut C) -> Option<Duration> {
        Some(Duration::from_millis(self.rng.gen_range(100..1000)))
    }

    fn update(&mut self, tick: Duration, matrix: &mut C) {
        self.last_tick = tick;

        let mut buf = Vec::with_capacity(<C as LedMatrix>::AREA);
        for _ in 0..<C as LedMatrix>::AREA {
            buf.push(if self.rng.gen_bool(0.3) {
                RGB8::new(self.rng.gen(), self.rng.gen(), self.rng.gen())
            } else {
                RGB8::new(0, 0, 0)
            })
        }
        matrix.set_buf(&mut buf);
    }
}
