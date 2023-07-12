use lux_camp_badge::led::{Animation, MatrixConfig};
use smart_leds::hsv::{hsv2rgb, Hsv};
use smart_leds_trait::{SmartLedsWrite, RGB8};
use std::time::Duration;

struct Inner {
    last_tick: Duration,
    fading_speed: Option<Duration>,
    hue: u8,
    step_size: u8,
}

impl Inner {
    fn new(step_size: u8, fading_speed: Option<Duration>) -> Self {
        Self {
            last_tick: Duration::ZERO,
            fading_speed,
            hue: 0,
            step_size,
        }
    }
}

/// Fill the entire screen with a fading rainbow.
pub struct FadingRainbow(Inner);

impl FadingRainbow {
    pub fn new(step_size: u8, fading_speed: Option<Duration>) -> Self {
        Self(Inner::new(step_size, fading_speed))
    }
}

impl<B, C: MatrixConfig<Backend = B>> Animation<C> for FadingRainbow
where
    B: SmartLedsWrite<Color = RGB8>,
{
    fn update(
        &mut self,
        tick: Duration,
    ) -> Option<Vec<<<C as MatrixConfig>::Backend as SmartLedsWrite>::Color>> {
        if let Some(amount) = self.0.fading_speed {
            crate::wait_at_least(amount, self.0.last_tick, tick)?;
        }

        self.0.last_tick = tick;
        self.0.hue += self.0.step_size; // Overflow is what we want here

        let hsv = Hsv {
            hue: self.0.hue,
            sat: 255,
            val: 25,
        };
        Some(vec![hsv2rgb(hsv); <C as MatrixConfig>::AREA])
    }
}

/// Fill the entire screen with a sliding rainbow.
pub struct SlidingRainbow(Inner);

impl SlidingRainbow {
    pub fn new(step_size: u8, fading_speed: Option<Duration>) -> Self {
        Self(Inner::new(step_size, fading_speed))
    }
}

impl<B, C: MatrixConfig<Backend = B>> Animation<C> for SlidingRainbow
where
    B: SmartLedsWrite<Color = RGB8>,
{
    fn update(
        &mut self,
        tick: Duration,
    ) -> Option<Vec<<<C as MatrixConfig>::Backend as SmartLedsWrite>::Color>> {
        if let Some(amount) = self.0.fading_speed {
            crate::wait_at_least(amount, self.0.last_tick, tick)?;
        }

        self.0.last_tick = tick;
        self.0.hue += self.0.step_size;

        let mut buf = Vec::with_capacity(<C as MatrixConfig>::AREA);
        for n in 0..<C as MatrixConfig>::AREA {
            buf.push(hsv2rgb(Hsv {
                hue: self.0.hue + (n as u8 * self.0.step_size),
                sat: 255,
                val: 25,
            }))
        }
        Some(buf)
    }
}
