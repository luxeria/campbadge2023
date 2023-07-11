use std::time::Duration;

use smart_leds_trait::RGB;

pub mod matrix;

pub type FrameBuf = Vec<RGB<u8>>;

pub trait MatrixSize {
    const X: usize;
    const Y: usize;
}

pub trait Animation {
    type Dimension: MatrixSize;

    fn init(&mut self) -> Option<FrameBuf> {
        None
    }

    fn update(&mut self, tick: Duration) -> Option<FrameBuf> {
        None
    }
}
