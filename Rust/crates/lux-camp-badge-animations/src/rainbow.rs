use lux_camp_badge::led::{hsv_rgb_convert::*, Animation, LedMatrix};
use smart_leds_trait::{SmartLedsWrite, RGB8};
use std::time::Duration;

struct Inner {
    fading_speed: Option<Duration>,
    hue: u8,
    step_size: u8,
}

impl Inner {
    fn new(step_size: u8, fading_speed: Option<Duration>) -> Self {
        Self {
            fading_speed,
            hue: 0,
            step_size,
        }
    }
}

/// Fill the entire screen with a fading rainbow.
pub struct Fade(Inner);

impl Fade {
    pub fn build<Matrix, Driver>(
        step_size: u8,
        fading_speed: Option<Duration>,
    ) -> Box<dyn Animation<Matrix> + Send>
    where
        Matrix: LedMatrix<Driver = Driver>,
        Driver: SmartLedsWrite<Color = RGB8>,
    {
        Box::new(Self(Inner::new(step_size, fading_speed)))
    }
}

impl<B, C: LedMatrix<Driver = B>> Animation<C> for Fade
where
    B: SmartLedsWrite<Color = RGB8>,
{
    fn init(&mut self, _matrix: &mut C) -> Option<Duration> {
        self.0.fading_speed
    }

    fn update(&mut self, _tick: Duration, matrix: &mut C) {
        self.0.hue += self.0.step_size; // Overflow is what we want here
        let hsv = Hsv8 {
            hue: self.0.hue,
            sat: 255,
            val: 255,
        };
        let buf = &mut vec![<Hsv8 as Hsv2Rgb>::hsv2rgb(hsv); <C as LedMatrix>::AREA];
        matrix.set_buf(buf);
    }
}

/// Fill the entire screen with a sliding rainbow.
pub struct Slide(Inner);

impl Slide {
    pub fn build<Matrix, Driver>(
        step_size: u8,
        fading_speed: Option<Duration>,
    ) -> Box<dyn Animation<Matrix> + Send>
    where
        Matrix: LedMatrix<Driver = Driver>,
        Driver: SmartLedsWrite<Color = RGB8>,
    {
        Box::new(Self(Inner::new(step_size, fading_speed)))
    }
}

impl<B, C: LedMatrix<Driver = B>> Animation<C> for Slide
where
    B: SmartLedsWrite<Color = RGB8>,
{
    fn init(&mut self, _matrix: &mut C) -> Option<Duration> {
        self.0.fading_speed
    }

    fn update(&mut self, _tick: Duration, matrix: &mut C) {
        self.0.hue += self.0.step_size;
        let mut buf = Vec::with_capacity(<C as LedMatrix>::AREA);
        for n in 0..<C as LedMatrix>::AREA {
            buf.push(<Hsv8 as Hsv2Rgb>::hsv2rgb(Hsv8 {
                hue: self.0.hue + (n as u8 * self.0.step_size),
                sat: 255,
                val: 255,
            }))
        }
        matrix.set_buf(&mut buf);
    }
}
