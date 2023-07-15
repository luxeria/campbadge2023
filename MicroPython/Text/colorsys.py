import math

def rgb_to_hsv(rgb_color):
    """Converts colors from the RGB color space to the HSV color space.

    Parameters
    ----------
    rgb_color : tuple (r, g, b)
        Color in the RGB color space

    Returns
    -------
    tuple (h, s, v)
        Color in the HSV color space

    """
    (r, g, b) = rgb_color
    r = float(1 / 255 * r)
    g = float(1 / 255 * g)
    b = float(1 / 255 * b)
    high = max(r, g, b)
    low = min(r, g, b)
    h, s, v = high, high, high

    d = high - low
    s = 0 if high == 0 else d/high

    if high == low:
        h = 0.0
    else:
        h = {
            r: (g - b) / d + (6 if g < b else 0),
            g: (b - r) / d + 2,
            b: (r - g) / d + 4,
        }[high]
        h /= 6

    return h, s, v

def hsv_to_rgb(hsv_color):
    """Converts colors from the HSV color space to the RGB color space.

    Parameters
    ----------
    hsv_color : tuple (h, s, v)
        Color in the HSV color space

    Returns
    -------
    tuple (r, g, b)
        Color in the RGB color space

    """
    (h, s, v) = hsv_color
    i = math.floor(h*6)
    f = h*6 - i
    p = v * (1-s)
    q = v * (1-f*s)
    t = v * (1-(1-f)*s)

    r, g, b = [
        (v, t, p),
        (q, v, p),
        (p, v, t),
        (p, q, v),
        (t, p, v),
        (v, p, q),
    ][int(i%6)]
    r = int(255 * r)
    g = int(255 * g)
    b = int(255 * b)
    return r, g, b