use lux_camp_badge::led::{Animation, LedMatrix, WriteLeds};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use smart_leds::RGB8;
use std::time::Duration;

/// Each pixel has a % probability of 30% getting colored randomly on each frame.
/// The frame rate is choosen at random between 100ms an 1s.
pub struct P30(SmallRng);

impl P30 {
    pub fn build<Matrix, Driver>(seed: u64) -> Box<dyn Animation<Matrix> + Send>
    where
        Matrix: LedMatrix<Driver = Driver>,
        Driver: WriteLeds<Color = RGB8>,
    {
        Box::new(Self(SmallRng::seed_from_u64(seed)))
    }
}

impl<B, C: LedMatrix<Driver = B>> Animation<C> for P30
where
    B: WriteLeds<Color = RGB8>,
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

/// Each pixel has a 50% chance of being off or on at each frame.
pub struct Flip<Matrix, Driver>
where
    Matrix: LedMatrix<Driver = Driver>,
    Driver: WriteLeds<Color = RGB8>,
{
    color: <<Matrix as LedMatrix>::Driver as WriteLeds>::Color,
    rng: SmallRng,
}

impl<Matrix: LedMatrix<Driver = Driver>, Driver> Flip<Matrix, Driver>
where
    Matrix: LedMatrix<Driver = Driver> + 'static,
    Driver: WriteLeds<Color = RGB8> + 'static,
{
    pub fn build(seed: u64) -> Box<dyn Animation<Matrix> + Send>
    where
        Matrix: LedMatrix<Driver = Driver>,
        Driver: WriteLeds<Color = RGB8>,
    {
        let mut rng = SmallRng::seed_from_u64(seed);
        let color = RGB8::new(rng.gen(), rng.gen(), rng.gen());
        Box::new(Self { color, rng })
    }
}

impl<Matrix, Driver, C: LedMatrix<Driver = Driver>> Animation<C> for Flip<Matrix, Driver>
where
    Matrix: LedMatrix<Driver = Driver>,
    Driver: WriteLeds<Color = RGB8>,
{
    fn init(&mut self, _matrix: &mut C) -> Option<Duration> {
        Some(Duration::from_secs(1))
    }

    fn update(&mut self, _tick: Duration, matrix: &mut C) {
        let mut buf = Vec::with_capacity(<C as LedMatrix>::AREA);
        for _ in 0..<C as LedMatrix>::AREA {
            buf.push(if self.rng.gen_bool(0.5) {
                self.color
            } else {
                RGB8::new(0, 0, 0)
            })
        }
        matrix.set_buf(&mut buf);
    }
}
