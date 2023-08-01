#include <Arduino.h>
#include <FastLED.h>
#include <WiFi.h>
#include "config.h"
#include "WebServer.h"
#include <ArduinoJson.h>

#include "json_post_handler.html"
#include "main.h"
#include "animation.h"

Animation* animation;

void setupRouting() {
  server.on("/", handleIndex);
  server.on("/animation", HTTP_POST, handleAnimation);
  server.on("/mode", HTTP_GET, handleMode);
  server.on("/brightness", HTTP_GET, handleBrightness);
  server.onNotFound(handleNotFound);
  server.begin();
}

void setupWifi() {
  Serial.print("Connecting to ");
  Serial.println(ssid);
  WiFi.begin(ssid, password);
  while (WiFi.status() != WL_CONNECTED) {
    delay(500);
    Serial.print(".");
  }
  Serial.println("");
  Serial.println("WiFi connected.");
  Serial.println("IP address: ");
  Serial.println(WiFi.localIP());
}

void setup() {
  Serial.begin(115200);
  
  pinMode(VCC_PIN,OUTPUT);
  digitalWrite(VCC_PIN,1);

  setupWifi();
  setupRouting();

  delay(10);

  FastLED.addLeds<NEOPIXEL, DATA_PIN>(leds, NUM_LEDS);  // GRB ordering assumed
  FastLED.setBrightness(25);

  animation  = new Rainbow();
}


void loop() {

  if (!WiFi.isConnected()){
    ESP.restart();
  }
  server.handleClient();

  if (mode == Mode::off) {
    FastLED.clear();
    FastLED.show();
    
  }
  else if(mode == Mode::animations) {
    animation->tick();
  }
  else if(mode == Mode::interactive){
    FastLED.show();
  };
}

void handleAnimation(){
  if (server.hasArg("plain") == false) {
  }
  String body = server.arg("plain");
  deserializeJson(jsonDocument, body);

  String animationType = jsonDocument["animation"];
  Serial.println("Animatiton:" + animationType);

  delete animation;
  if (animationType=="rainbow") animation = new Rainbow();
  if (animationType=="snake") animation = new Snake();
  if (animationType=="strobo") animation = new Strobo();


  server.send(200, "text/plain", "");
}

void handleMode(){
  String modeType = server.arg("set");
  Serial.println("Mode: " + modeType);
  if (modeType == "animation")    mode = Mode::animations;
  if (modeType == "interactive" ) mode = Mode::interactive;
  if (modeType == "off")          mode = Mode::off;

  server.send(200, "text/plain", "");
}

void handleBrightness(){
  int brightness = server.arg("val").toInt();
  Serial.println("Brightness: " + server.arg("val"));
  FastLED.setBrightness(brightness);

  server.send(200, "text/plain", "");
}
void handleNotFound(){
  server.send(404, "text/plain", "Not found");
}

void handleIndex(){
  server.send(200, "text/html", INDEX_HTML);
}