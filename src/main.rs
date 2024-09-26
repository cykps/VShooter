mod key_logger;
mod mode;

use anyhow::Result;
use key_logger::{Key, KeyLogger};
use rppal::i2c::I2c;
use ssd1306::{mode::BufferedGraphicsMode, prelude::*, I2CDisplayInterface, Ssd1306};
use std::{thread::sleep, time::Duration};

struct Player {
    x: u8,
    y: u8,
}

fn main() -> Result<(), std::convert::Infallible> {
    // Create styles used by the drawing operations.

    //// Create a new simulator display with 64x64 pixels.
    //let output_settings = OutputSettingsBuilder::new()
    //    .theme(BinaryColorTheme::OledBlue)
    //    .build();
    //let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(128, 64));
    //let mut window = Window::new("Progress", &output_settings);

    // Initialize Display
    let i2c = I2c::new().unwrap();
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    // Key Input
    let mut key_logger = KeyLogger::new();

    mode::progress(&mut display).unwrap();

    sleep(Duration::from_millis(500));

    mode::shouting(&mut display, &mut key_logger).unwrap();

    loop {
        sleep(Duration::from_millis(10));
        println!("{:?}", key_logger.get_unread_keys());
    }

    Ok(())
}
