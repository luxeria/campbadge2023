
# 2Mbaud, 7 data bits, no parity, one stop bit -- this lines up with
# the WS2812 protocol well enough, at 3 bits per bit...
stty -F /dev/ttyUSB0 2000000 cs7 -parenb

# some basic sequences
ON='\x5b\x5b\x5b\x5b\x5b\x5b\x5b\x5b'
OFF='\x12\x12\x12\x12\x12\x12\x12\x12'

# The UART will send 0xxxxxxx1 (7 data bits, start and stop bit)
# We use 3 uart bits per WS2812 bit (2MBaud). So there are 3 WS2812
# bits in one uart character. We can only send inverted WS2812 data
# signals, since the uart idles at 1 and we can't send a long
# sequence of zeros due to the stop bits, so the reset signal will
# need to be a long stretch of 1 (idle), and we rely on an inverter
# in the data line to make this WS2812 compatible.
# The valid bit patterns are 001 or 011, so we only send sequences
# like 0x10x10x1. If we assume 1 (that is 0) for the WS2812 data bits,
# the uart data bits work out to 0x12.
base_pattern=0x12

# One LED takes 24 bits. This works out well, since it divides by
# 3 into 8 uart characters.

pack_led_bits() {
  r=$1
  g=$2
  b=$3

  all_bits=$((($b << 16) + ($r << 8) + $g))
  all_bits=$((~$all_bits)) # because we make an inverted signal
  bit_idx=-3 # the data bits go at idx 0, 3, 6
  current_word=$base_pattern
  for i in {1..24}
  do
    current_bit=$(($all_bits&1))
    all_bits=$(($all_bits>>1))
    bit_idx=$(($bit_idx+3))

    current_word=$(($current_word + ($current_bit * (1<<$bit_idx))))

    if [ $bit_idx == 6 ]
    then
      echo -en '\x'$(printf %x $current_word)
      bit_idx=-3
      current_word=$base_pattern
    fi
  done
}

test_pattern='
rrgbw
rgbbw
gbbbw
_____
wwwww
'

text_to_bits() {
  for row
  do
    for i in {1..5}
    do
      case $row in
        r*)
          pack_led_bits 255 0 0
          ;;
        g*)
          pack_led_bits 0 255 0
          ;;
        b*)
          pack_led_bits 0 0 255
          ;;
        w*)
          pack_led_bits 255 255 255
          ;;
        _*)
          pack_led_bits 0 0 0
          ;;
      esac
      row=${row#?}
    done
  done
}


main () {
  while true
  do
    # very basic test/PoC
    dd if=/dev/urandom of=/dev/ttyUSB0 bs=1K count=1
    sleep 1
    # the intermediate result must be buffered to make sure it is sent without
    # any gaps (that would be interpreted as a reset)
    bits="$(text_to_bits $pattern)"
    echo -n "$bits" > /dev/ttyUSB0
    sleep 1  
  done
}

main
