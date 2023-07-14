use std::time::Duration;

use smart_leds_trait::SmartLedsWrite;

pub mod matrix;

/// Configuration trait for implementing the LED matrix being used.
///
/// Types implementing this trait want to use an internal framebuffer.
/// Each LED matrix can have a different routing or physical layout of their LEDs,
/// Where animations just want to write pixels to `X` and `Y` (and `Z` for 3D LED cubes) coordinates.
/// It is then up to the implementation of this trait to ensure a particular write to some coordinate
/// will be made correctly in the internal frame buffer.
///
/// The returned value of the [read] function is supplied forward to the [SmartLedsWrite::write] function.
pub trait LedMatrix: 'static {
    /// The X dimension of the matrix
    const X: usize;
    /// The Y dimension of the matrix
    const Y: usize;
    /// The Z dimension, in case your animation is for an LED cube.
    /// 2D LED Matrices can set this to zero.
    const Z: usize;
    /// The total area of the matrix. Usually this is X * Y.
    /// This information is required often;
    /// Accessing it directly is faster than calculating X * Y every time.
    const AREA: usize;

    /// The driver for the LED matrix.
    type Backend: SmartLedsWrite;

    /// Read the entire internal frame buffer.
    fn read_buf(&self) -> Vec<<Self::Backend as SmartLedsWrite>::Color>;

    /// Write to the entire internal frame buffer.
    fn set_buf(&mut self, buf: Vec<<Self::Backend as SmartLedsWrite>::Color>);

    /// Write a pixel to the given `x` / `y` coordinate of your 2D LED Matrix.
    fn set_2d(&mut self, x: usize, y: usize, color: <Self::Backend as SmartLedsWrite>::Color);

    /// Write a pixel to the given `x` / `y` `z` coordinate of your LED Cube.
    /// 2D LED matrices don't need to implement this function, it'll default to set_2D.
    #[allow(unused)]
    fn set_3d(
        &mut self,
        x: usize,
        y: usize,
        z: usize,
        color: <Self::Backend as SmartLedsWrite>::Color,
    ) {
        Self::set_2d(self, x, y, color)
    }
}

/// Trait for implementing animations that can run on a variety of LED matrices.
///
/// State should be stored in `self`, where as properties of the LED matrix can be found
/// on the [MatrixConfig] type.
///
/// Many examples can be found in the `lux-camp-badge-animations` crate.
#[allow(unused)]
pub trait Animation<C: LedMatrix> {
    /// Initialization function for your Animation.
    /// Useful for clearing the frame buf or drawing a static image.
    /// If `false` is returned, nothing will be drawed upon initialization.
    fn init(&mut self, matrix: &mut C) -> bool {
        false
    }

    /// The draw function of your Animation, called at every frame.
    /// If `false` is returned, nothing will be drawed for the current frame.
    fn update(&mut self, tick: Duration, matrix: &mut C) -> bool {
        false
    }
}
