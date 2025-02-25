# MPU6050-DMP Examples

This directory contains examples demonstrating use of the MPU9250 sensor.

## Examples

### [Basic](src/basic.rs)

Basic example showing how to:

- Initialize the sensor with I2C
- Perform sensor calibration
- Read accelerometer, gyroscope and temperature data

## Building and Running

1. Connect your MPU6050 to a Raspberry Pi Pico 2:
   - SDA -> GP16
   - SCL -> GP17
   - VCC -> 3.3V
   - GND -> GND

2. Build and flash an example:

   ```bash
   # For the basic example
   cargo run --example basic
   ```
