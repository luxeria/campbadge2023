use std::time::Duration;

pub mod rainbow;
pub mod random;
pub mod static_scene;

/// Check if `now` is more than `interval` after `since`.
/// Useful helper if you only want to draw at for example every `interval` milliseconds.
pub fn skip_frame(interval: Duration, since: Duration, now: Duration) -> bool {
    since + interval > now
}
