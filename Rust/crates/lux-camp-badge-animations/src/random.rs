use esp_idf_svc::systime::EspSystemTime;
use lux_camp_badge::led::{Animation, FrameBuf, MatrixSize};
use smart_leds_trait::RGB8;
use std::time::Duration;

/// In a (pseudo) random interval of max. 1 second,
/// color exactly one pixel with a (psuedo) random color.
pub struct RandomAnimation(Duration);

impl Default for RandomAnimation {
    fn default() -> Self {
        Self(EspSystemTime {}.now())
    }
}

impl<C: MatrixSize> Animation<C> for RandomAnimation {
    fn update(&mut self, tick: Duration) -> Option<FrameBuf> {
        let interval = Duration::from_millis(tick.as_micros() as u64 % 1000);
        if self.0 + interval > tick {
            return None;
        }
        self.0 = tick;

        let mut buf = vec![RGB8::new(0, 0, 0); <C as MatrixSize>::AREA];
        let seed = self.0.as_micros() as u64;
        buf[seed as usize % <C as MatrixSize>::AREA] = RGB8::new(
            ((seed + interval.as_micros() as u64) % 255) as u8,
            (seed % 255) as u8,
            ((seed - 122) % 255) as u8,
        );

        buf.into()
    }
}
