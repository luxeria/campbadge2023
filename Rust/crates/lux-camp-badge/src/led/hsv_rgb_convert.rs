//! A generic version of `smart_leds::hsv::hsv2rgb`.
use smart_leds_trait::RGB8;

/// 8bit HSV value.
#[derive(Copy, Clone, Default)]
pub struct Hsv8 {
    pub hue: u8,
    pub sat: u8,
    pub val: u8,
}

/// Convert HSV values into RGB values.
pub trait Hsv2Rgb {
    type Rgb;
    type Hsv;

    fn hsv2rgb(hsv: Self::Hsv) -> Self::Rgb;
}

/// Convert RGB values into HSv values.
pub trait Rgb2Hsv {
    type Rgb;
    type Hsv;

    fn rgb2hsv(rgb: Self::Rgb) -> Self::Hsv;
}

impl Hsv2Rgb for Hsv8 {
    type Rgb = RGB8;
    type Hsv = Self;

    /// Converts a hsv value into RGB values. Because the hsv values are integers, the precision of the
    /// resulting RGB value is limited to +- 4
    fn hsv2rgb(hsv: Self::Hsv) -> Self::Rgb {
        let v: u16 = hsv.val as u16;
        let s: u16 = hsv.sat as u16;
        let f: u16 = (hsv.hue as u16 * 2 % 85) * 3; // relative interval

        let p: u16 = v * (255 - s) / 255;
        let q: u16 = v * (255 - (s * f) / 255) / 255;
        let t: u16 = v * (255 - (s * (255 - f)) / 255) / 255;
        match hsv.hue {
            0..=42 => RGB8 {
                r: v as u8,
                g: t as u8,
                b: p as u8,
            },
            43..=84 => RGB8 {
                r: q as u8,
                g: v as u8,
                b: p as u8,
            },
            85..=127 => RGB8 {
                r: p as u8,
                g: v as u8,
                b: t as u8,
            },
            128..=169 => RGB8 {
                r: p as u8,
                g: q as u8,
                b: v as u8,
            },
            170..=212 => RGB8 {
                r: t as u8,
                g: p as u8,
                b: v as u8,
            },
            213..=254 => RGB8 {
                r: v as u8,
                g: p as u8,
                b: q as u8,
            },
            255 => RGB8 {
                r: v as u8,
                g: t as u8,
                b: p as u8,
            },
        }
    }
}

impl Rgb2Hsv for RGB8 {
    type Rgb = Self;
    type Hsv = Hsv8;

    fn rgb2hsv(rgb: Self::Rgb) -> Self::Hsv {
        const EPS: f32 = 0.0001;

        let r = rgb.r as f32 / 255.0;
        let g = rgb.g as f32 / 255.0;
        let b = rgb.b as f32 / 255.0;

        let min = r.min(g).min(b);
        let max = r.max(g).max(b);
        let delta = max - min;

        let hue = match () {
            _ if delta < EPS || max < EPS => 0.,
            _ if max == r => (6. + (g - b) / delta) % 6.,
            _ if max == g => (b - r) / delta + 2.,
            _ if max == b => (r - g) / delta + 4.,
            _ => 0.,
        } / 6.;

        let sat = if max < EPS { 0. } else { delta / max };

        let val = max;

        Hsv8 {
            hue: (hue * 255.) as u8,
            sat: (sat * 255.) as u8,
            val: (val * 255.) as u8,
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    fn distance(i: u8, j: u8) -> u8 {
        if i < j {
            j - i
        } else {
            i - j
        }
    }
    #[rustfmt::skip]
    const RGB: [RGB8; 16] = [
        RGB8 { r: 255, g: 0  , b:   0},
        RGB8 { r: 255, g: 127, b:   0},
        RGB8 { r: 255, g: 255, b:   0},
        RGB8 { r: 127, g: 255, b:   0},
        RGB8 { r:   0, g: 255, b:   0},
        RGB8 { r:   0, g: 255, b: 127},
        RGB8 { r:   0, g: 255, b: 255},
        RGB8 { r:   0, g: 127, b: 255},
        RGB8 { r:   0, g: 0  , b: 255},
        RGB8 { r: 127, g: 0  , b: 255},
        RGB8 { r: 255, g: 0  , b: 255},
        RGB8 { r: 255, g: 0  , b: 127},
        RGB8 { r: 255, g: 0  , b:   0},
        RGB8 { r:  19, g:  35, b:  29},
        RGB8 { r: 137, g: 137, b: 136},
        RGB8 { r:   4, g:  41, b:   8},
    ];

    #[rustfmt::skip]
    const HSV: [Hsv8; 16] = [
        Hsv8{hue:   0, sat: 255, val: 255},
        Hsv8{hue:  21, sat: 255, val: 255},
        Hsv8{hue:  42, sat: 255, val: 255},
        Hsv8{hue:  64, sat: 255, val: 255},
        Hsv8{hue:  85, sat: 255, val: 255},
        Hsv8{hue: 106, sat: 255, val: 255},
        Hsv8{hue: 127, sat: 255, val: 255},
        Hsv8{hue: 149, sat: 255, val: 255},
        Hsv8{hue: 170, sat: 255, val: 255},
        Hsv8{hue: 191, sat: 255, val: 255},
        Hsv8{hue: 212, sat: 255, val: 255},
        Hsv8{hue: 234, sat: 255, val: 255},
        Hsv8{hue: 255, sat: 255, val: 255},
        Hsv8{hue: 111, sat: 123, val:  35},
        Hsv8{hue:  21, sat:   3, val: 138},
        Hsv8{hue:  89, sat: 230, val:  42},
    ];

    #[test]
    fn test_hsv2rgb_1() {
        for i in 0..HSV.len() {
            let new_hsv = <Hsv8 as Hsv2Rgb>::hsv2rgb(HSV[i]);
            assert!(distance(new_hsv.r, RGB[i].r) < 4);
            assert!(distance(new_hsv.g, RGB[i].g) < 4);
            assert!(distance(new_hsv.b, RGB[i].b) < 4);
        }
    }

    #[test]
    // if sat == 0 then all colors are equal
    fn test_hsv2rgb_2() {
        for i in 0..=255 {
            let rgb = <Hsv8 as Hsv2Rgb>::hsv2rgb(Hsv8 {
                hue: i,
                sat: 0,
                val: 42,
            });
            assert! {rgb.r == rgb.b};
            assert! {rgb.b == rgb.g};
        }
    }

    #[test]
    fn identity() {
        for c in RGB {
            assert_eq!(<Hsv8 as Hsv2Rgb>::hsv2rgb(<RGB8 as Rgb2Hsv>::rgb2hsv(c)), c);
        }
    }
}
