use lux_camp_badge::led::{Animation, LedMatrix};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use smart_leds_trait::{SmartLedsWrite, RGB8};
use std::time::Duration;

/// Each pixel has a % probability of 30% getting colored randomly on each frame.
/// The frame rate is choosen at random between 100ms an 1s.
pub struct PerlinAnimation(SmallRng);

impl PerlinAnimation {
    pub fn build<Matrix, Driver>(seed: u64) -> Box<dyn Animation<Matrix> + Send>
        where
            Matrix: LedMatrix<Driver = Driver>,
            Driver: SmartLedsWrite<Color = RGB8>,
    {
        Box::new(Self(SmallRng::seed_from_u64(seed)))
    }
}

impl<B, C: LedMatrix<Driver = B>> Animation<C> for PerlinAnimation
    where
        B: SmartLedsWrite<Color = RGB8>,
{
    fn init(&mut self, _matrix: &mut C) -> Option<Duration> {
        Some(Duration::from_millis(self.0.gen_range(100..1000)))
    }

    fn update(&mut self, _tick: Duration, matrix: &mut C) {
        let mut buf = Vec::with_capacity(<C as LedMatrix>::AREA);
        for _ in 0..<C as LedMatrix>::AREA {
            buf.push(if self.0.gen_bool(0.3) {
                RGB8::new(self.0.gen(), self.0.gen(), self.0.gen())
            } else {
                RGB8::new(0, 0, 0)
            })
        }
        matrix.set_buf(&mut buf);
    }
}
