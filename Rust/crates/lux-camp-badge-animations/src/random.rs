use esp_idf_svc::systime::EspSystemTime;
use lux_camp_badge::led::{Animation, MatrixConfig};
use smart_leds_trait::{SmartLedsWrite, RGB8};
use std::time::Duration;

/// In a (pseudo) random interval of max. 1 second,
/// color exactly one pixel with a (psuedo) random color.
pub struct RandomAnimation(Duration);

impl Default for RandomAnimation {
    fn default() -> Self {
        Self(EspSystemTime {}.now())
    }
}

impl<B, C: MatrixConfig<Backend = B>> Animation<C> for RandomAnimation
where
    B: SmartLedsWrite<Color = RGB8>,
{
    fn update(
        &mut self,
        tick: Duration,
    ) -> Option<Vec<<<C as MatrixConfig>::Backend as smart_leds_trait::SmartLedsWrite>::Color>>
    {
        let interval = Duration::from_millis(tick.as_micros() as u64 % 1000);
        if self.0 + interval > tick {
            return None;
        }
        self.0 = tick;

        let mut buf = vec![RGB8::new(0, 0, 0); <C as MatrixConfig>::AREA];
        let seed = self.0.as_micros() as u64;
        buf[seed as usize % <C as MatrixConfig>::AREA] = RGB8::new(
            ((seed + interval.as_micros() as u64) % 255) as u8,
            (seed % 255) as u8,
            ((seed - 122) % 255) as u8,
        );

        buf.into()
    }
}
