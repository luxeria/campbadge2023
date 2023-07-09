use smart_leds::hsv::Hsv;
use smart_leds_trait::{RGB, RGB8};

pub fn hsv2rgb(hsv: Hsv) -> RGB8 {
    let v: u16 = hsv.val as u16;
    let s: u16 = hsv.sat as u16;
    let f: u16 = (hsv.hue as u16 * 2 % 85) * 3; // relative interval

    let p: u16 = v * (255 - s) / 255;
    let q: u16 = v * (255 - (s * f) / 255) / 255;
    let t: u16 = v * (255 - (s * (255 - f)) / 255) / 255;
    match hsv.hue {
        0..=42 => RGB {
            r: v as u8,
            g: t as u8,
            b: p as u8,
        },
        43..=84 => RGB {
            r: q as u8,
            g: v as u8,
            b: p as u8,
        },
        85..=127 => RGB {
            r: p as u8,
            g: v as u8,
            b: t as u8,
        },
        128..=169 => RGB {
            r: p as u8,
            g: q as u8,
            b: v as u8,
        },
        170..=212 => RGB {
            r: t as u8,
            g: p as u8,
            b: v as u8,
        },
        213..=254 => RGB {
            r: v as u8,
            g: p as u8,
            b: q as u8,
        },
        255 => RGB {
            r: v as u8,
            g: t as u8,
            b: p as u8,
        },
    }
}

pub fn rgb2hsv(rgb: RGB8) -> Hsv {
    const EPS: f32 = 0.0001;

    let r = rgb.r as f32 / 255.0;
    let g = rgb.g as f32 / 255.0;
    let b = rgb.b as f32 / 255.0;

    let min = r.min(g).min(b);
    let max = r.max(g).max(b);
    let delta = max - min;

    let hue = match () {
        _ if delta < EPS || max < EPS => 0.,
        _ if max == r => (6.+ (g - b)/delta) % 6.,
        _ if max == g => (b - r)/delta + 2.,
        _ if max == b => (r - g)/delta + 4.,
        _ => 0.
    } / 6.;

    let sat = if max < EPS {
        0.
    } else {
        delta / max
    };

    let val = max;

    Hsv {
        hue: (hue * 255.) as u8,
        sat: (sat * 255.) as u8,
        val: (val * 255.) as u8,
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_identity() {
        assert_eq!(hsv2rgb(rgb2hsv(RGB8::new(255, 0, 0))), RGB8::new(255, 0, 0));
        assert_eq!(hsv2rgb(rgb2hsv(RGB8::new(0, 255, 0))), RGB8::new(0, 255, 0));
        assert_eq!(hsv2rgb(rgb2hsv(RGB8::new(0, 0, 255))), RGB8::new(0, 0, 255));
    }
}
