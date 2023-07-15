use lux_camp_badge::led::{Animation, LedMatrix};
extern crate fuss;
use fuss::Simplex;
use smart_leds_trait::{SmartLedsWrite, RGB8};
use std::time::Duration;


pub struct PerlinAnimation(Simplex);

impl PerlinAnimation {
    pub fn build<Matrix, Driver>() -> Box<dyn Animation<Matrix> + Send>
        where
            Matrix: LedMatrix<Driver = Driver>,
            Driver: SmartLedsWrite<Color = RGB8>,
    {
        Box::new(Self(Simplex::new()))
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
        /*for _ in 0..<C as LedMatrix>::AREA {
            buf.push(if self.0.gen_bool(0.3) {
                RGB8::new(self.0.gen(), self.0.gen(), self.0.gen())
            } else {
                RGB8::new(0, 0, 0)
            })
        }*/
        matrix.set_buf(&mut buf);
    }
}
