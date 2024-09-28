mod key_logger;
mod mode;
mod progress_ring;

use anyhow::Result;
use key_logger::KeyLogger;
use rppal::i2c::I2c;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};
use std::{thread::sleep, time::Duration};

fn main() -> Result<(), std::convert::Infallible> {
    // Initialize Display
    let i2c = I2c::new().unwrap();
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    // Key Input
    let mut key_logger = KeyLogger::new();

    progress_ring::run(&mut display).unwrap();

    sleep(Duration::from_millis(500));

    mode::shouting(&mut display, &mut key_logger).unwrap();

    loop {
        sleep(Duration::from_millis(10));
        println!("{:?}", key_logger.get());
    }
}
