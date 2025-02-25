//! Embassy Async MPU6050-DMP Example
//!
//! This example demonstrates using the MPU6050 sensor with Embassy's async runtime on a Raspberry Pi Pico.
//! It shows how to:
//! - Initialize the sensor with async I2C
//! - Load and initialize the DMP firmware
//! - Perform sensor calibration
//! - Continuously read accelerometer, gyroscope and temperature data
//!
//! Hardware Setup:
//! - Connect MPU6050 to Raspberry Pi Pico:
//!   - SDA -> GP14
//!   - SCL -> GP15
//!   - VCC -> 3.3V
//!   - GND -> GND

#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_rp::{block::ImageDef, config::Config, i2c::InterruptHandler};
use embassy_time::{Delay, Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

// mpu9250
use mpu9250::Mpu9250;

embassy_rp::bind_interrupts!(struct Irqs {
    I2C0_IRQ => InterruptHandler<embassy_rp::peripherals::I2C0>;
});

/// Firmware image type for bootloader
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

/// Firmware entry point
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Config::default());

    // Initialize the MPU9250 sensor using I2C0
    let sda = p.PIN_16;
    let scl = p.PIN_17;
    let config = embassy_rp::i2c::Config::default();
    let bus = embassy_rp::i2c::I2c::new_async(p.I2C0, scl, sda, Irqs, config);

    let mut delay = Delay;

    let mut sensor = Mpu9250::imu_default(bus, &mut delay).unwrap();
    info!("Sensor Initialized");

    // Connection test
    let who_am_i = sensor.who_am_i().unwrap();
    info!("WHO_AM_I: 0x{:x}", who_am_i);

    // let mag_who_am_i = sensor.ak8963_who_am_i().unwrap();
    // info!("AK8963 WHO_AM_I: 0x{:x}", mag_who_am_i);

    info!("Test measurement before calibration...");
    let all = sensor.all::<[f32; 3]>().unwrap();
    info!(
        "Accel XYZ(m/s^2): {} {} {} | Gyro XYZ (rad/s): {} {} {} | Temp (C): {}",
        all.accel[0],
        all.accel[1],
        all.accel[2],
        all.gyro[0],
        all.gyro[1],
        all.gyro[2],
        all.temp
    );

    info!("Calibrating sensor...");
    let accel_biases =
        sensor.calibrate_at_rest::<_, [f32; 3]>(&mut delay).unwrap();
    info!(
        "Calibration complete. Accel biases: {} {} {}",
        accel_biases[0], accel_biases[1], accel_biases[2]
    );

    info!("Test measurement after calibration...");
    let all = sensor.all::<[f32; 3]>().unwrap();
    info!(
        "Accel XYZ(m/s^2): {} {} {} | Gyro XYZ (rad/s): {} {} {} | Temp (C): {}",
        all.accel[0] - accel_biases[0],
        all.accel[1] - accel_biases[1],
        all.accel[2] - accel_biases[2],
        all.gyro[0],
        all.gyro[1],
        all.gyro[2],
        all.temp
    );

    Timer::after(Duration::from_millis(1000)).await;
    info!("Starting data read loop...");

    loop {
        let all = sensor.all::<[f32; 3]>().unwrap();
        // info!(
        //     "Accel XYZ(m/s^2): {} {} {} | Gyro XYZ (rad/s): {} {} {} | Mag Field XYZ(uT): {} {} {} | Temp (C): {}",
        //     all.accel[0],
        //     all.accel[1],
        //     all.accel[2],
        //     all.gyro[0],
        //     all.gyro[1],
        //     all.gyro[2],
        //     all.mag[0],
        //     all.mag[1],
        //     all.mag[2],
        //     all.temp
        // );

        info!(
            "Accel XYZ(m/s^2): {} {} {} | Gyro XYZ (rad/s): {} {} {} | Temp (C): {}",
            all.accel[0] - accel_biases[0],
            all.accel[1] - accel_biases[1],
            all.accel[2] - accel_biases[2],
            all.gyro[0],
            all.gyro[1],
            all.gyro[2],
            all.temp
        );

        Timer::after(Duration::from_millis(1000)).await;
    }
}
