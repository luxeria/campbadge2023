use std::time::Duration;

use smart_leds_trait::SmartLedsWrite;

pub mod matrix;

pub trait MatrixConfig: 'static {
    const X: usize;
    const Y: usize;
    const AREA: usize;
    type Backend: SmartLedsWrite;
}

pub trait Animation<C: MatrixConfig> {
    fn init(&mut self) -> Option<Vec<<C::Backend as SmartLedsWrite>::Color>> {
        None
    }

    fn update(&mut self, _tick: Duration) -> Option<Vec<<C::Backend as SmartLedsWrite>::Color>> {
        None
    }
}
