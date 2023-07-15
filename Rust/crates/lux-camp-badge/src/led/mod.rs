use std::time::Duration;

use smart_leds_trait::SmartLedsWrite;

pub mod hsv_rgb_convert;
pub mod matrix;

pub type Color<T> = <<T as LedMatrix>::Driver as SmartLedsWrite>::Color;

/// Configuration trait for implementing the LED matrix being used.
///
/// Types implementing this trait want to use an internal framebuffer.
/// Each LED matrix can have a different routing or physical layout of their LEDs,
/// Where animations just want to write pixels to `X` and `Y` (and `Z` for 3D LED cubes) coordinates.
/// It is then up to the implementation of this trait to ensure a particular write to some coordinate
/// will be made correctly in the internal frame buffer.
///
/// The returned value of the [LedMatrix::read_buf] function is copied to the [SmartLedsWrite::write] function.
pub trait LedMatrix {
    /// The X dimension of the matrix
    const X: usize;
    /// The Y dimension of the matrix
    const Y: usize;
    /// The Z dimension, in case your animation is for an LED cube (optional).
    /// 2D LED Matrices can set this to zero.
    const Z: usize = 0;
    /// The total area of the matrix. Usually this is X * Y.
    /// This information is required often; accessing it directly is faster.
    const AREA: usize = Self::X * Self::Y;
    /// The total volume of the cube. Usually this is X * Y * Z.
    const VOLUME: usize = Self::X * Self::Y * Self::Z;

    /// The driver for the LED matrix.
    type Driver: SmartLedsWrite;

    /// Read the entire internal frame buffer.
    fn read_buf(&self) -> &[Color<Self>];

    /// Write to the entire internal frame buffer.
    fn set_buf(&mut self, buf: &mut [Color<Self>]);

    /// Write a pixel to the given `x` / `y` coordinate of your 2D LED Matrix.
    fn set_2d(&mut self, x: usize, y: usize, color: &Color<Self>);

    /// Write a pixel to the given `x` / `y` `z` coordinate of your LED Cube.
    /// 2D LED matrices don't need to implement this function, it'll default to set_2D.
    #[allow(unused)]
    fn set_3d(&mut self, x: usize, y: usize, z: usize, color: &Color<Self>) {
        Self::set_2d(self, x, y, color)
    }
}

/// Trait for implementing animations that can run on a variety of LED matrices.
///
/// State should be stored in `self`, where as properties of the LED matrix can be found
/// on the `MatrixConfig` type.
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
    /// If `false` is returned, nothing will be drawed in the current frame.
    fn update(&mut self, tick: Duration, matrix: &mut C) -> bool {
        false
    }
}
