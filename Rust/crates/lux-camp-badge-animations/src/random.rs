use lux_camp_badge::led::{Animation, LedMatrix};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use smart_leds_trait::{SmartLedsWrite, RGB8};
use std::time::Duration;

/// Every 250ms, each pixel has a 30% chance of getting colored randomly.
pub struct RandomAnimation {
    last_tick: Duration,
    rng: SmallRng,
}

impl Default for RandomAnimation {
    fn default() -> Self {
        Self {
            last_tick: Duration::ZERO,
            rng: SmallRng::seed_from_u64(123),
        }
    }
}

impl<B, C: LedMatrix<Backend = B>> Animation<C> for RandomAnimation
where
    B: SmartLedsWrite<Color = RGB8>,
{
    fn update(&mut self, tick: Duration, matrix: &mut C) -> bool {
        if crate::wait_for(Duration::from_millis(250), self.last_tick, tick).is_none() {
            return false;
        }
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
        true
    }
}
