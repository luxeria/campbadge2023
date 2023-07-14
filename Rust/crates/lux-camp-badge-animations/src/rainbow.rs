use lux_camp_badge::led::{hsv_rgb_convert::*, Animation, LedMatrix};
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

    // Helper that returns true if the current frame should be skipped.
    // Otherwise last_tick is updated to the current one.
    fn skip_frame(&mut self, tick: Duration) -> bool {
        if self
            .fading_speed
            .map(|amount| crate::wait_for(amount, self.last_tick, tick))
            .is_none()
        {
            return true;
        }

        self.last_tick = tick;
        false
    }
}

/// Fill the entire screen with a fading rainbow.
pub struct FadingRainbow(Inner);

impl FadingRainbow {
    pub fn new(step_size: u8, fading_speed: Option<Duration>) -> Self {
        Self(Inner::new(step_size, fading_speed))
    }
}

impl<B, C: LedMatrix<Backend = B>> Animation<C> for FadingRainbow
where
    B: SmartLedsWrite<Color = RGB8>,
{
    fn update(&mut self, tick: Duration, matrix: &mut C) -> bool {
        if self.0.skip_frame(tick) {
            return false;
        };

        self.0.hue += self.0.step_size; // Overflow is what we want here
        let hsv = Hsv8 {
            hue: self.0.hue,
            sat: 255,
            val: 25,
        };
        let buf = &mut vec![<Hsv8 as Hsv2Rgb>::hsv2rgb(hsv); <C as LedMatrix>::AREA];
        matrix.set_buf(buf);
        true
    }
}

/// Fill the entire screen with a sliding rainbow.
pub struct SlidingRainbow(Inner);

impl SlidingRainbow {
    pub fn new(step_size: u8, fading_speed: Option<Duration>) -> Self {
        Self(Inner::new(step_size, fading_speed))
    }
}

impl<B, C: LedMatrix<Backend = B>> Animation<C> for SlidingRainbow
where
    B: SmartLedsWrite<Color = RGB8>,
{
    fn update(&mut self, tick: Duration, matrix: &mut C) -> bool {
        if self.0.skip_frame(tick) {
            return false;
        };

        self.0.hue += self.0.step_size;
        let mut buf = Vec::with_capacity(<C as LedMatrix>::AREA);
        for n in 0..<C as LedMatrix>::AREA {
            buf.push(<Hsv8 as Hsv2Rgb>::hsv2rgb(Hsv8 {
                hue: self.0.hue + (n as u8 * self.0.step_size),
                sat: 255,
                val: 25,
            }))
        }
        matrix.set_buf(&mut buf);
        true
    }
}
