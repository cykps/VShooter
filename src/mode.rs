use anyhow::Result;
use device_query::{DeviceQuery, DeviceState};
use embedded_graphics::mono_font::MonoTextStyleBuilder;
use embedded_graphics::{
    mono_font::{
        ascii::{FONT_10X20, FONT_6X10},
        MonoTextStyle,
    },
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Arc, PrimitiveStyleBuilder, StrokeAlignment},
    text::{Alignment, Baseline, Text, TextStyleBuilder},
};
use rppal::i2c::I2c;
use serde::ser::StdError;
use serde::Serialize;
use ssd1306::{mode::BufferedGraphicsMode, prelude::*, I2CDisplayInterface, Ssd1306};
use std::{borrow::BorrowMut, fs::OpenOptions};
use std::{
    sync::{Arc as A, Mutex as M},
    thread,
    time::Duration,
};

use crate::key_logger::KeyLogger;

struct Player {
    x: u8,
    y: u8,
}

impl Player {
    fn new(x: u8, y: u8) -> Self {
        Player { x, y }
    }
}

pub fn shouting(
    mut display: Ssd1306<
        I2CInterface<I2c>,
        DisplaySize128x64,
        BufferedGraphicsMode<DisplaySize128x64>,
    >,
    key_logger: &mut KeyLogger,
) -> Result<
    //Ssd1306<I2CInterface<I2c>, DisplaySize128x64, BufferedGraphicsMode<DisplaySize128x64>>,
    (),
> {
    let mut player1 = Player::new(10, 32);
    let mut player1 = Player::new(128 - 10, 32);

    Ok(())
}

pub fn progress(
    display: &mut Ssd1306<
        I2CInterface<I2c>,
        DisplaySize128x64,
        BufferedGraphicsMode<DisplaySize128x64>,
    >,
) -> Result<
    //Ssd1306<I2CInterface<I2c>, DisplaySize128x64, BufferedGraphicsMode<DisplaySize128x64>>,
    (),
> {
    // The current progress percentage

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    Text::with_baseline("Hello world!", Point::zero(), text_style, Baseline::Top)
        .draw(display)
        .unwrap();

    Text::with_baseline("Hello Rust!", Point::new(0, 16), text_style, Baseline::Top)
        .draw(display)
        .unwrap();

    display.flush().unwrap();

    let arc_stroke = PrimitiveStyleBuilder::new()
        .stroke_color(BinaryColor::On)
        .stroke_width(5)
        .stroke_alignment(StrokeAlignment::Inside)
        .build();
    let character_style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
    let text_style = TextStyleBuilder::new()
        .baseline(Baseline::Middle)
        .alignment(Alignment::Center)
        .build();

    // The current progress percentage
    let mut progress = 78;

    for progress in 0..=100 {
        display.clear(BinaryColor::Off).unwrap();
        let sweep = progress as f32 * 360.0 / 100.0;

        // Draw an arc with a 5px wide stroke.
        Arc::new(Point::new(2 + 32, 2), 64 - 4, 90.0.deg(), sweep.deg())
            .into_styled(arc_stroke)
            .draw(display)
            .unwrap();

        // Draw centered text.
        let text = format!("{}%", progress);
        Text::with_text_style(
            &text,
            display.bounding_box().center(),
            character_style,
            text_style,
        )
        .draw(display)
        .unwrap();

        display.flush().unwrap();

        thread::sleep(Duration::from_millis(10));
    }
    Ok(())
}
