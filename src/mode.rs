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
use rppal::i2c::I2c;
use serde::ser::StdError;
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

pub fn progress<D: DrawTarget<Color = BinaryColor>>(
    display: &mut D,
    window: &mut Window,
) -> Result<()>
where
    <D as DrawTarget>::Error: Send,
    <D as DrawTarget>::Error: Sync,
    <D as DrawTarget>::Error: StdError,
    <D as embedded_graphics::draw_target::DrawTarget>::Error: 'static,
{
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

    'running: loop {
        display.clear(BinaryColor::Off);

        let sweep = progress as f32 * 360.0 / 100.0;

        // Draw an arc with a 5px wide stroke.
        Arc::new(Point::new(32 + 2, 2), 64 - 4, 90.0.deg(), sweep.deg())
            .into_styled(arc_stroke)
            .draw(display)?;

        // Draw centered text.
        let text = format!("{}%", progress);
        Text::with_text_style(
            &text,
            display.bounding_box().center(),
            character_style,
            text_style,
        )
        .draw(display)?;

        //window.update(display);

        if window.events().any(|e| e == SimulatorEvent::Quit) {
            break 'running Ok(());
        }
        thread::sleep(Duration::from_millis(50));

        progress = (progress + 1) % 101;
    }
}
