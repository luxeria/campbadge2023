use std::time::Duration;

pub mod rainbow;
pub mod random;

/// Check if `now` is more than `amount` after `since`.
/// Useful helper if you only want to draw at for example every N milliseconds.
///
/// Returns `None` if not enough time has passed yet.
/// Otherwise returns the lag.
pub fn wait_at_least(amount: Duration, since: Duration, now: Duration) -> Option<Duration> {
    let threshold = since + amount;
    if threshold <= now {
        return Some(now - threshold);
    }
    None
}
