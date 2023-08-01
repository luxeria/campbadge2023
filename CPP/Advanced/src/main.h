#ifndef _MAIN_H
#define _MAIN_H

#define NUM_LEDS 25
#define DATA_PIN 1
#define VCC_PIN 0
#define DIMX 5
#define DIMY 5

const char* ssid = SSID;
const char* password = PASSWORD;
WebServer server(80);

StaticJsonDocument<250> jsonDocument;
char buffer[250];

enum Mode {animations, interactive, off};
Mode mode = Mode::animations;

CRGB leds[NUM_LEDS];


void handleIndex();
void handleAnimation();
void handleMode();
void handleBrightness();
void handleInteractiveMode();
void handleNotFound();
void setupRouting();
void setupWifi();

void setup();
void loop();

#endif