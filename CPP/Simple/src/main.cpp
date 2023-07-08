#include <Arduino.h>
#include <FastLED.h>

// How many leds in your strip?
#define NUM_LEDS 25

#define DATA_PIN 10

// Define the array of leds
CRGB leds[NUM_LEDS];

void setup() {
  // put your setup code here, to run once:
    FastLED.addLeds<NEOPIXEL, DATA_PIN>(leds, NUM_LEDS);  // GRB ordering is assumed
    FastLED.setBrightness(25);
}

void loop() {
  // Turn the LED on, then pause
  for (int i=0;i<NUM_LEDS;i++){
    leds[i] = CRGB::Red;
  }
  FastLED.show();
  delay(500);
  // Now turn the LED off, then pause
  for (int i=0;i<NUM_LEDS;i++){
    leds[i] = CRGB::Black;
  }
  FastLED.show();
  delay(500);
}
