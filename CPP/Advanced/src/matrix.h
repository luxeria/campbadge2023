#ifndef _MATRIX_H
#define _MATRIX_H

#include <FastLED.h>

class Matrix
{
    private:
        CRGB* mleds;
        int dx;
        int dy;

        int getIndex(int x, int y){
            return (y*dx + (dx-1-x));
        }
    public:

        Matrix(CRGB* mleds, int dx, int dy){
            this->mleds = mleds;
            this->dx = dx;
            this->dy = dy;
        }

        void set(int x,  int y, CRGB color ) {
            if (x >= dx || y >= dy) return;
            mleds[getIndex(x,y)] = color;
        }

        CRGB get(int x, int y){
            return mleds[getIndex(x,y)];
        }

};

#endif