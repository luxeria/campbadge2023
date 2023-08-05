Bash Support
============

_Microcontrollers are hard. I don't know any proper programming languages. Can't we just do this as a shell script?_

**Yes.**


How?
---

I found a very cheap generic USB UART adapter (CH340G based). This can be used to flip bits in the real world.

The WS2812 interface is very much not a UART. But: This USB UART chip (and most others as well) supports much higher bitrates than one might expect. In this case, the highest (documented) is 2Mbaud (and that is even called "standard"), which is *just* high enough. If we use 3 UART bits for one WS2812 symbol, the sequence 100 is 0.5us/1us (barely in spec), similar for 110. The hard part is that the UART has some fixed protocol bits that we can't freely control.

But, UARTs (or rather RS232) has some weird modes, such as character leghts different from 8bits. If we choose 7bit characters and no parity, one charater is sent as 9bits (start + 7 data + stop) which is exactly 3 WS2812 bits.

There is another issue: We can't send a long sequence of 0's for the reset signal. The UART idles high, and the start bits are always 0. So, we need to invert the signal. This has the nice effect that now start and stop bits perfectly line up with the WS2812 symbols, we only need to insert some more framing in the data bits.

### Hardware Mod

So we need an extra inverter. Luckily, the level shifter is implemented as a discrete MOSFET which we can abuse to build an inverter by connecting the input to the gate, and source to ground, with a pullup on the output (drain). The latter are already connected correctly, so all it takes is swapping gate and source, and making the VCC pad GND. This can be done by desoldering the MOSFET, bending the pins up, flipping it upside down and soldering it on again. Then bridge VCC to GND instead of +5V or whatever.

An alternative option might be the RS232 mode of the UART chip (there is a physical pin that can be pulled up to invert some signals, but I have not tried that). It might also be possible to do that from software with other USB UARTs (FTDI) to get away without any hardware modification.

### Software

Now, we just need to set the serial to the correct mode and blast out the correct bits. Packing the bits is quite straightforward on a sheet of paper, and doing it in bash is totally possible, even if bash is one of the least suitable langauages for this purpose. It's almost all builtins, so it's not even that slow.

Performance is uncritical: We prepare the bit sequence into a buffer, and then write that to the UART at full speed. after the transmission ends, the WS2812 will see a reset symbol and show the result. Attempting to do the write in more than one operation will likely cause spurious resets, so don't do that.
