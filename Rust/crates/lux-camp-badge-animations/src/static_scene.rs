use lux_camp_badge::led::{Animation, MatrixConfig};
use smart_leds_trait::{SmartLedsWrite, RGB8};

/// Draw a static image to the LED matrix.
pub struct Scene(pub Vec<RGB8>);

impl<B, C: MatrixConfig<Backend = B>> Animation<C> for Scene
where
    B: SmartLedsWrite<Color = RGB8>,
{
    fn init(&mut self) -> Option<Vec<<<C as MatrixConfig>::Backend as SmartLedsWrite>::Color>> {
        self.0.clone().into()
    }
}
