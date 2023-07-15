import framebuf
import os
import re

from micropython import const

_XBM_WIDTH = re.compile(r'#define\s\w+_width\s(\d+)')
_XBM_HEIGHT = re.compile(r'#define\s\w+_height\s(\d+)')
_XBM_BITS = re.compile(r'\w+_bits\[\] = \{([^}]+)\}')
_XBM_SPLIT = re.compile(r'\s*,\s*')

_WHITE = (255, 255, 255)
_BLACK = (0, 0, 0)

class Symbol:
    def __init__(self, path):
        with open(path) as f:
            xbm = f.read()
        w = _XBM_WIDTH.search(xbm)
        h = _XBM_HEIGHT.search(xbm)
        if w is None or h is None:
            raise RuntimeError("xbm: invalid size")
        b = _XBM_BITS.search(xbm)
        if b is None:
            raise RuntimeError("xbm: invalid bits")

        bits = bytearray(int(s, 0) for s in _XBM_SPLIT.split(b.group(1).strip()) if s)

        self.w = int(w.group(1), 0)
        self.h = int(h.group(1), 0)
        self.buf = framebuf.FrameBuffer(bits, self.w, self.h,  framebuf.MONO_HMSB)
        self.remaining_w = self.w

    def width(self):
        return self.remaining_w

    def height(self):
        return self.h

    def scroll(self):
        self.remaining_w = max(self.remaining_w - 1, 0)

    def pixel(self, x, y):
        x += self.w - self.remaining_w
        return _WHITE if self.buf.pixel(x, y) else _BLACK

class Marquee:
    def __init__(self, files):
        self.load_queue = files
        self.draw_queue = []

    def _scroll(self):
        if not self.load_queue and not self.draw_queue:
            return False

        # scroll first symbol in draw queue by one pixel
        if len(self.draw_queue) > 0:
            self.draw_queue[0].scroll()
            if self.draw_queue[0].width() == 0:
                self.draw_queue.pop(0)
        return True

    def _fill_queue(self, canvas):
        draw_queue_width = sum(sym.width() for sym in self.draw_queue)
        while self.load_queue and draw_queue_width < canvas.width():
            sym = Symbol(self.load_queue.pop(0))
            self.draw_queue.append(sym)
            draw_queue_width += sym.width()

    def update(self, canvas):
        # load symbols if needed
        self._fill_queue(canvas)

        # draw symbols in draw queue
        column = 0
        for sym in self.draw_queue:
            # remaining size of current symbol
            width = min(sym.width(), canvas.width() - column)
            height = min(sym.height(), canvas.height())

            # draw symbol slice
            for x in range(width):
                for y in range(height):
                    canvas.pixel(column + x, y, sym.pixel(x, y))

            # continue drawing at end of current symbol
            column += width
            if column >= canvas.width():
                break

        # clear trail
        for x in range(column, canvas.width()):
            for y in range(canvas.height()):
                canvas.pixel(column, y, _BLACK)
            column += 1

        # flush drawing
        canvas.draw()

        return self._scroll()

_TOKEN_LETTER = const(1)
_TOKEN_ICON = const(2)

def _tokenize(s):
    open = False
    name = ""
    for char in s:
        if not open:
            # opening {
            if char == '{':
                name = ""
                open = True
            # regular character
            else:
                yield (_TOKEN_LETTER, char)
        else:
            # closing }
            if char == '}':
                open = False
                yield (_TOKEN_ICON, name)
            # escaped {
            elif char == '{' and not name:
                open = False
                yield (_TOKEN_LETTER, char)
            # collect name
            else:
                name += char
    if open:
        raise ValueError("unmatched { in template")


def text(text, font = "symbols"):
    files = []
    for (ty, tok) in _tokenize(text):
        if ty == _TOKEN_LETTER:
            file = "{}/letter_{}.xbm".format(font, ord(tok))
        elif ty == _TOKEN_ICON:
            file = "{}/{}.xbm".format(font, tok)
        else:
            raise ValueError("invalid token")
        try:
            os.stat(file)
        except:
            raise ValueError('invalid symbol "{}"'.format(tok))
        files.append(file)

    return Marquee(files)