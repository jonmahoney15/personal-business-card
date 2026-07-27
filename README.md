# Personal Business Card

ESP32-C3 powered business card with a MAX7219 LED matrix displaying a Snake game.

## Firmware
The app is written in Rust. It makes use of `esp_hal` and the `MAX7219` crate.

The embassy framework is used as the runtime enabling async workflows.

## Hardware

| Component | Description |
|-----------|-------------|
| ESP32-C3 | Wi-Fi/BLE microcontroller running the firmware |
| MAX7219 | LED matrix driver IC |
| LED Matrix 8x8 | 64-LED display used for the Snake game |
| AMS1117 | 3.3V linear voltage regulator |
| USB-C | Power and programming interface |

