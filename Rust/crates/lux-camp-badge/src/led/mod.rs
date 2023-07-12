use std::time::Duration;

use smart_leds_trait::SmartLedsWrite;

pub mod matrix;

/// Configuration trait describing the LED matrix being used.
pub trait MatrixConfig: 'static {
    /// The X dimension of the matrix
    const X: usize;
    /// The Y dimension of the matrix
    const Y: usize;
    /// The total area of the matrix. Usually this is X * Y.
    /// This information is required often;
    /// Accessing it directly is faster than calculating X * Y every time.
    const AREA: usize;
    /// The driver for the LED matrix.
    type Backend: SmartLedsWrite;
}

/// Trait for implementing animations that can run on a variety of LED matrices.
///
/// State should be stored in `self`, where as properties of the LED matrix can be found
/// on the [MatrixConfig] type.
///
/// Many examples can be found in the `lux-camp-badge-animations` crate.
pub trait Animation<C: MatrixConfig> {
    /// Initialization function for your Animation. The output of it will be drawed
    /// whenever this animation is loaded.
    ///
    /// If `None` is returned, nothing will be drawed.
    fn init(&mut self) -> Option<Vec<<C::Backend as SmartLedsWrite>::Color>> {
        None
    }

    /// The draw function of your Animation, called at every frame.
    /// The output of this function will be written to the matrix.
    ///
    /// If `None` is returned, nothing will be drawed for the current frame.
    fn update(&mut self, _tick: Duration) -> Option<Vec<<C::Backend as SmartLedsWrite>::Color>> {
        None
    }
}
