# MicroPython Beispiele - lux-camp-badge-2023

## Übersicht

### [Simple](Simple/main.py)

Ein kleines Beispielprogramm welches alle Pixel mit einer zufälligen Farbe
leuchten lässt.

### [Clock](Clock/main.py)

Eine [binäre Uhr](https://de.wikipedia.org/wiki/Bin%C3%A4re_Uhr), die erste
Spalte zeigt die aktuelle Stunde, die zweite und dritte Spalte zeigen die
Minute und die vierte und fünfte Spalte zeigen die Sekunden. Die initiale
Uhrzeit wird über WLAN und NTP geholt.

### [Text](Text/main.py)

Ein Lauftext-Display. Zum Installieren folgendene Befehle ausführen:

    mpremote cp -r symbols/ :
    mpremote cp colorsys.py :
    mpremote cp marquee.py :
    mpremote cp main.py :

Die einzelnen Buchstaben der Schrift können im `symbols` Ordner via GIMP o.ä.
bearbeitet werden.

## Beispielprogramme ausführen

Flashe zuerst eine aktuelle Micropython-Version auf dein Board. Für den M5Stamp
C3U findest du unten eine Anleitung.

Danach installiere das
[`mpremote`](https://docs.micropython.org/en/latest/reference/mpremote.html) Tool.
Damit kannst du die Beispielprogramme wie folgt auf das Board kopieren:

    mpremote cp main.py :.

Alternativ kannst du auch Snippets direkt in die serielle Konsole kopieren,
in dem du über `CTRL+E` in den Einfügemodus wechselst, dein Beispielprogramm
aus der Zwischenablage hineinkopierst, und dann die Eingabe mit `CTRL+D`
abschliesst. Die serielle Konsole öffnest du wie folgt:

    mpremote repl


## Micropython auf den M5Stamp C3U flashen

Folgende Anleitung erklärt, wie man MicroPython auf den M5Stamp C3U flashen kann.

### Download-Modus aktiveren

Um den Download Mode zu aktivieren, drücke und halte den mittleren Button (G9)
währenddem die Stromversorgung gekappt wird. Halte dazu den mittleren Button
gedrückt während du den Reset Button drückst, _oder_ halte den mittleren Button
gedrückt während du das USB Kabel aus- und wieder einsteckst.

Wenn du erfolgreich bist, solltest du ein neues USB CDC ACM Gerät auf deinem
Laptop erkennen.

### Micropython Firmware installieren

Lade den [ESP32-C3 with USB](https://micropython.org/download/esp32c3-usb/)
Micropython Port herunter.

Während das Board im aktiven Download-Modus ist (siehe oben),
führe folgenden Befehl aus:

```
esptool.py --chip esp32c3 --port /dev/ttyACM0 --baud 460800 \
    write_flash -z 0x0 esp32c3-usb-20230426-v1.20.0.bin
```

Warte bis die Firmware fertig geflasht ist, und starte das Board neu. Danach
solltest du die Micropython Konsole über die serielle Schnittstelle erreichen.

