use std::time::Duration;

use lux_camp_badge::led::{Animation, LedMatrix};
use smart_leds_trait::{SmartLedsWrite, RGB8};

/// Draw a static image to the LED matrix.
pub struct Scene<Color, const X: usize, const Y: usize>(pub [[Color; Y]; X]);

impl<const X: usize, const Y: usize> Default for Scene<RGB8, X, Y>
where
    RGB8: Default + Copy,
{
    fn default() -> Self {
        Self([[Default::default(); Y]; X])
    }
}

impl<Color: Default, const X: usize, const Y: usize, B, C: LedMatrix<Driver = B>> Animation<C>
    for Scene<Color, X, Y>
where
    B: SmartLedsWrite<Color = Color>,
{
    fn init(&mut self, matrix: &mut C) -> Option<Duration> {
        for y in 0..<C as LedMatrix>::Y {
            for x in 0..<C as LedMatrix>::X {
                matrix.set_2d(x, y, &self.0[y][x])
            }
        }
        Some(Duration::MAX)
    }
}
