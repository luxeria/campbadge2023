#ifndef _ANIMATION_H
#define _ANIMATION_H

#include "main.h"
#include <FastLED.h>

class Animation
{
    public:
        virtual void tick() = 0;
    protected:
        const int frameLength;
        unsigned long mili = 0;

        Animation(int count, int length)
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

class RainbowAnimation: public Animation {
    private:
        int ticker = 0;
    public:
        void tick() {

            if (!frameFinished()) return;

            int color;
            if (ticker%2) color = CRGB::Purple;
            else color = CRGB::HTMLColorCode::Green;
            
            for (int i=0;i<NUM_LEDS;i++){
                leds[i] =  color;
            }
            FastLED.show();

            ticker = (++ticker)%2;
        };
        RainbowAnimation()
            : Animation(2, 500)
        {};
};

class Snake: public Animation {
    private:
        const int length = 5;
        int position = 0;
    public:
        void tick() {

            if (!frameFinished()) return;
            
            FastLED.clear();

            // for (int i=position;i<(position+length);i++){
            //     leds[i%NUM_LEDS] =  CRGB::Blue;
            // }
            leds[position] = CRGB::Blue;

            position=(++position)%NUM_LEDS;
            FastLED.show();
        };

        Snake()
            : Animation(2, 500)
        {};
};


#endif