import machine
import neopixel
import time
import marquee
import colorsys

# Grove Port A on M5Stamp C3U
vcc = machine.Pin(0, machine.Pin.OUT)
din = machine.Pin(1, machine.Pin.OUT)

# 25 NeoPixels, turn on reference voltage pin
vcc.on()

class Matrix:
    def __init__(self, pin, width, height):
        self.np = neopixel.NeoPixel(pin, width * height)
        self.w = width
        self.h = height

    def width(self):
        return self.w

    def height(self):
        return self.h

    def pixel(self, x, y, c):
        self.np[(self.w * self.h - 1) - (x + y * self.h)] = c

    def draw(self):
        self.np.write()

class Rainbow:
    def __init__(self, canvas, brightness=1.0):
        self.hue = 0
        self.value = brightness
        self.canvas = canvas

    def width(self):
        return self.canvas.width()

    def height(self):
        return self.canvas.height()

    def _cycle(self):
        self.hue = (self.hue + 1) % 255
        return self.hue / 255

    def pixel(self, x, y, c):
        _, _, v = colorsys.rgb_to_hsv(c)
        if v > 0:
            c = colorsys.hsv_to_rgb((self._cycle(), 1.0, self.value * v))
        self.canvas.pixel(x, y, c)

    def draw(self):
        return self.canvas.draw()

# Rainbow colors the monochrome drawing calls from the Marquee class
canvas = Rainbow(Matrix(din, 5, 5), brightness=0.5)

m = marquee.text("Welcome to Lux Camp 2023 !!!")
while m.update(canvas):
    time.sleep(0.1)
