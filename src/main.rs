mod key_logger;
mod mode;

use anyhow::Result;
use chrono::prelude::*;
use device_query::{DeviceQuery, DeviceState};
use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Arc, PrimitiveStyleBuilder, StrokeAlignment},
    text::{Alignment, Baseline, Text, TextStyleBuilder},
};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use key_logger::{Key, KeyLogger};
use rppal::i2c::I2c;
use serde::Serialize;
use serde_json;
use ssd1306::{mode::BufferedGraphicsMode, prelude::*, I2CDisplayInterface, Ssd1306};
use std::io::Write;
use std::{borrow::BorrowMut, fs::OpenOptions};
use std::{
    sync::{Arc as A, Mutex as M},
    thread,
    time::{Duration, Instant},
};

struct Player {
    x: u8,
    y: u8,
}

fn main() -> Result<(), std::convert::Infallible> {
    // Create styles used by the drawing operations.
    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();

    // Create a new simulator display with 64x64 pixels.
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(128, 64));
    let mut window = Window::new("Progress", &output_settings);

    //let i2c = I2c::new().unwrap();
    //let interface = I2CDisplayInterface::new(i2c);
    //let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
    //    .into_buffered_graphics_mode();

    // Key Input
    let key_logger = KeyLogger::new();

    mode::progress(&mut display, &mut window);

    Ok(())
}
