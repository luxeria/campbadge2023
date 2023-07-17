use std::time::Duration;

use rgb::RGB8;

use self::hsv_rgb_convert::Hsv8;

pub mod hsv_rgb_convert;
pub mod matrix;

#[cfg(feature = "smart-leds-trait")]
pub mod smart_led_write;

pub type Color<T> = <<T as LedMatrix>::Driver as WriteLeds>::Color;

pub trait WriteLeds {
    type Error;
    type Color;

    fn write(&mut self, buf: &[Self::Color]) -> Result<(), Self::Error>;
}

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
    type Driver: WriteLeds;

    /// Read the entire internal frame buffer.
    fn read_buf(&self) -> &[Color<Self>]
    where
        Color<Self>: Dimmable;

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

/// RGB or HSV values are dimmable. Usually, full send will bleach your eyes out anyways.
pub trait Dimmable {
    /// Dimm `self` according to `level`, where level is a divisor of the full brightness.
    #[allow(unused)]
    fn dimm(&mut self, level: u8) {}
}

impl Dimmable for RGB8 {
    fn dimm(&mut self, level: u8) {
        self.r /= level;
        self.g /= level;
        self.b /= level;
    }
}

impl Dimmable for Hsv8 {
    fn dimm(&mut self, level: u8) {
        self.val /= level;
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
    ///
    /// The return value can be used to set the frame rate of the animation:
    /// For example, if you want your animation to update only once per second,
    /// return a duration of 1 second.
    /// Return `None` if the animation should updated at the FPS rate of the
    /// matrix (once per frame time, default 24 FPS).
    fn init(&mut self, matrix: &mut C) -> Option<Duration> {
        None
    }

    /// The draw function of your Animation, called at every frame.
    fn update(&mut self, tick: Duration, matrix: &mut C) {}
}
