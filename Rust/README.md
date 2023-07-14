# Rust Beispiele - lux-camp-badge-2023

## [Einfaches Beispiel](examples/Simple/)
 * LEDs ansteuern
 * Farben umrechnen

(Rust setup siehe unten)
Falls Rust und esp32-toolchain bereits läuft:
```
cargo run --release --bin simple # flash the simple example
```


## [Komplexeres Beispiel](examples/Advanced/)
 * Beispiel Bilder
 * Animationen
 * Webserver mit UI um LEDs anzusteuern

```
cargo run --release --bin advanced # flash the advanced example
```

## Toolchain für Embedded Rust installieren
### Annahme:
* Rust ist installiert
* Rustup ist installiert

### Abhängigkeiten Installieren

Target für Rust installieren
```
rustup target add riscv32imc-unknown-none-elf
```

Tools installieren um code für den ESP zu kompilieren, projekte zu erstellen und den ESP zu flaschen
```
cargo install cargo-generate
 cargo install ldproxy
 cargo install espup
 cargo install espflash
 ```

ESP-Toolchain installieren
```
espup install
 ```

Bemerkung: Je nachdem muss das Terminal oder die IDE neu gestartet werden. Unter VSCode musste ich "export-esp.ps1" manuell ausführen, damit die Umgebungsvariablen korrekt gesetzt werden.


im file ```.cargo\config.toml``` wird ein custom runner definiert, welcher den ESP flashed und die Serielle Schnittstelle autom. monitored.
Der ESP kann also mit folgendem Befhel einfach geflashed werden:
```
cargo run --release
```
Examples ausführen:
```
cargo run --release --bin simple # flash the simple example
cargo run --release --bin advanced # flash the advanced example
```


## Optional: Neues Projekt erstellen

 ```
  cargo generate --vcs none --git https://github.com/esp-rs/esp-idf-template cargo
  ```
