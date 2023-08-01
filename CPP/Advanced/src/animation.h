#ifndef _ANIMATION_H
#define _ANIMATION_H

#include "main.h"
#include <FastLED.h>

class Animation
{
    public:
        virtual void tick() = 0;
    protected:
        int frameLength;
        unsigned long mili = 0;

        Animation(int length)
            : frameLength(length)
        {
            FastLED.clear();
            FastLED.show();
            delay(500);
        }
        bool frameFinished() {
            unsigned long new_milis = millis();
            if (new_milis-mili < frameLength) return false;
            mili = new_milis;
            return true;
        }
};

class Rainbow: public Animation {
    private:
        int color = 0;
        int dColor = 1;
    public:
        void tick() {

            if (!frameFinished()) return;
            FastLED.showColor(CHSV(color, 255, 255));
            color += dColor;
        };
        Rainbow()
            : Animation(50)
        {};
};

class Snake: public Animation {
    private:
        int length = (NUM_LEDS+3);
        const int order[NUM_LEDS+3] = { 0, 1, 2, 3, 4,
                                      9, 8, 7, 6, 5,
                                     10,11,12,13,14,
                                     19,18,17,16,15,
                                     20,21,22,23,24,
                                     -1,-1,-1};
        int position[6] = {25,26,27,0,1,2};
        int color[6] = {
            CRGB::HTMLColorCode::LightCyan,
            CRGB::HTMLColorCode::SkyBlue,
            CRGB::HTMLColorCode::DeepSkyBlue,
            CRGB::HTMLColorCode::Blue,
            CRGB::HTMLColorCode::MidnightBlue,
            CRGB::HTMLColorCode::LightSteelBlue
                        };
    public:
        void tick() {

            if (!frameFinished()) return;
            
            FastLED.clear();
            for (int i=0; i<6; i++) {
                int pos = order[position[i]];
                if (pos>-1) leds[pos] = color[i];
            }
            for  (int i=0; i<5; i++) {
                position[i] = position[i+1];
            }
            position[5] = (position[5]+1)%length;
            FastLED.show();

        };

        Snake()
            : Animation(150)
        {};
};

class Strobo: public Animation {
    private:
        int length = (NUM_LEDS+3);
    public:
        void tick() {

            if (!frameFinished()) return;

            int brightness = FastLED.getBrightness();

            if (frameLength>50) frameLength*=0.9;
            else frameLength = 2000;

            FastLED.setBrightness(200);
            FastLED.showColor(CRGB::HTMLColorCode::White);
            delay(50);
            FastLED.setBrightness(brightness);
            FastLED.clear();
            FastLED.show();


        };

        Strobo()
            : Animation(2000)
        {};
};

#endif