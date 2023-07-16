// This is common when iterating over 2D matrices. In our context,
// using iterators instead, makes the code less readable for no benefits.
#![allow(clippy::needless_range_loop)]
/// Game of life, you know the rules.
pub mod gol;
/// Animations based on noise functions
pub mod noise;
/// Rainbow animations (classic HSV)
pub mod rainbow;
/// Animations that draw pixels randomly
pub mod random;
/// Static images or a set of static images played at constant frame rate
pub mod scene;

/// All available animation modules.
pub mod prelude {
    pub use crate::gol;
    pub use crate::noise;
    pub use crate::rainbow;
    pub use crate::random;
    pub use crate::scene;
}
