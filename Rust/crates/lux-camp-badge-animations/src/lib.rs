/// Rainbow animations (classic HSV)
pub mod rainbow;
/// Animations that draw pixels randomly
pub mod random;
/// Static images or a set of static images played at constant frame rate
pub mod scene;

/// All available animation modules.
pub mod prelude {
    pub use crate::rainbow;
    pub use crate::random;
    pub use crate::scene;
}
