import machine
import neopixel
import network
import ntptime
import time

class EUTimezone:
    def __init__(self, offset):
        self.offset = offset

    def localtime(self, dt=None):
        if dt is None:
            dt = time.gmtime()
        year, _, _, _, _, _, _, _ = dt
        # European DST starts last Sunday of March 1:00 UTC
        start = time.mktime((year, 3, 31 - (5 * year // 4 + 4) % 7, 1, 0, 0, 0, 0))
        # European DST ends last Sunday of October 1:00 UTC
        end = time.mktime((year, 10, 31 - (5 * year // 4 + 1) % 7, 1, 0, 0, 0, 0))
        # Seconds since epoch
        now = time.mktime(dt)

        offset = self.offset
        if start <= now < end:
            offset += 1 # day light savings time
        return time.gmtime(now + offset * 3600)

class LEDClock:
    def __init__(self, pin, width, height):
        self.np = neopixel.NeoPixel(pin, width * height)
        self.w = width
        self.h = height

    def _set_column(self, col, val, color=(255, 255, 255)):
        for i in range(self.h):
            bit_is_set = val & (1<<i)
            c = color if bit_is_set else (0, 0, 0)
            self.np[(i * self.w) + col] = c

    def update(self, hour, minute, second):
        self._set_column(4, hour, (20, 40, 30))
        self._set_column(3, minute // 10, (20, 10, 50))
        self._set_column(2, minute % 10, (25, 0, 40))
        self._set_column(1, second // 10, (50, 0, 20))
        self._set_column(0, second % 10, (40, 0, 10))
        self.np.write()

# We use Grove Port A on M5Stamp C3U
vcc = machine.Pin(0, machine.Pin.OUT)
din = machine.Pin(1, machine.Pin.OUT)

# 5x5 NeoPixels
clock = LEDClock(din, 5, 5)
# Turn on reference voltage pin
vcc.on()

# WiFi Settings
WIFI_ESSID = "INSERT YOUR ESSID HERE"
WIFI_PASSWORD = "INSERT YOUR PASSWORD HERE"
TIMEZONE = EUTimezone(1) # Europe/Zurich

# Fetch current time via WiFi
wifi = network.WLAN(network.STA_IF)
if not wifi.isconnected():
    wifi.active(True)
    wifi.connect(WIFI_ESSID, WIFI_PASSWORD)
    while not wifi.isconnected():
        time.sleep_ms(100)
ntptime.settime()

# Fetch local time and update clock face in loop
while True:
    _, _, _, hour, minute, second, _, _ = TIMEZONE.localtime()
    clock.update(hour, minute, second)
    time.sleep(1)