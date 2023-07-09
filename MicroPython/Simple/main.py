import machine, neopixel, random, time

# Grove Port A on M5Stamp C3U
vcc = machine.Pin(0, machine.Pin.OUT)
din = machine.Pin(1, machine.Pin.OUT)

# 25 NeoPixels, turn on reference voltage pin
np = neopixel.NeoPixel(din, 25)
vcc.on()

# Displays 5 random colors for 1 second each
for i in range(5):
    r = random.randint(0, 255)
    g = random.randint(0, 255)
    b = random.randint(0, 255)
    np.fill((r, g, b))
    np.write()
    time.sleep(1)