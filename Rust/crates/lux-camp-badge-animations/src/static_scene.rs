use lux_camp_badge::led::{Animation, LedMatrix};
use smart_leds_trait::{SmartLedsWrite, RGB8};

/// Draw a static image to the LED matrix.
#[derive(Default)]
pub struct Scene(pub Vec<RGB8>);

impl<B, C: LedMatrix<Backend = B>> Animation<C> for Scene
where
    B: SmartLedsWrite<Color = RGB8>,
{
    fn init(&mut self, matrix: &mut C) -> bool {
        if self.0.len() != <C as LedMatrix>::AREA {
            return false;
        }
        matrix.set_buf(self.0.clone());
        true
    }
}
