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
    primitives::{Arc, PrimitiveStyleBuilder, Rectangle, StrokeAlignment, Triangle},
    text::{Alignment, Baseline, Text, TextStyleBuilder},
};
use rppal::i2c::I2c;
use serde::ser::StdError;
use serde::Serialize;
use ssd1306::{mode::BufferedGraphicsMode, prelude::*, I2CDisplayInterface, Ssd1306};
use std::fmt::Binary;
use std::{borrow::BorrowMut, fs::OpenOptions};
use std::{
    sync::{Arc as A, Mutex as M},
    thread,
    time::Duration,
};

use crate::key_logger::KeyLogger;

struct Player {
    x: i32,
    y: i32,
    step: i32,
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        let step = 2;
        Self { x, y, step }
    }

    fn transfer(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }

    fn step(&mut self, x: i32, y: i32) {
        self.transfer(x * self.step, y * self.step);
    }

    fn draw(
        &mut self,
        display: &mut Ssd1306<
            I2CInterface<I2c>,
            DisplaySize128x64,
            BufferedGraphicsMode<DisplaySize128x64>,
        >,
    ) {
        let style = PrimitiveStyleBuilder::new()
            .fill_color(BinaryColor::On)
            .build();

        Rectangle::new(Point::new(self.x, self.y), Size::new(5, 5))
            .into_styled(style)
            .draw(display)
            .unwrap();
    }
}

#[derive(PartialEq)]
enum Direction {
    Right,
    Left,
}

struct Bullet {
    x: i32,
    y: i32,
    speed: i32,
    direction: Direction,
}

impl Bullet {
    fn new(x: i32, y: i32, direction: Direction) -> Self {
        let speed = 1;
        Self {
            x,
            y,
            speed,
            direction,
        }
    }

    fn transfer(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }

    fn fly(&mut self) {
        self.transfer(self.speed, 0);
    }

    fn draw(
        &mut self,
        display: &mut Ssd1306<
            I2CInterface<I2c>,
            DisplaySize128x64,
            BufferedGraphicsMode<DisplaySize128x64>,
        >,
    ) {
        let style = PrimitiveStyleBuilder::new()
            .fill_color(BinaryColor::On)
            .build();
        if self.direction == Direction::Right {
            Triangle::new(
                Point::new(self.x, self.y),
                Point::new(self.x - 4, self.y + 1),
                Point::new(self.x - 4, self.y - 1),
            )
            .into_styled(style)
            .draw(display)
            .unwrap();
        } else {
            Triangle::new(
                Point::new(self.x, self.y),
                Point::new(self.x + 4, self.y + 1),
                Point::new(self.x + 4, self.y - 1),
            )
            .into_styled(style)
            .draw(display)
            .unwrap();
        }
    }
}

pub fn shouting(
    display: &mut Ssd1306<
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
    let mut player2 = Player::new(128 - 10, 32);
    let mut bullets: Vec<Bullet> = Vec::new();

    loop {
        display.clear(BinaryColor::Off).unwrap();

        let keys = key_logger.get_unread_keys();
        for key_conf in keys {
            for key in key_conf.keyboard {
                println!("{:?}", key);
                match key.as_str() {
                    "F" => player1.step(1, 0),
                    "D" => player1.step(-1, 0),
                    "C" => player1.step(0, 1),
                    "R" => player1.step(0, -1),
                    "J" => player2.step(-1, 0),
                    "K" => player2.step(1, 0),
                    "I" => player2.step(0, -1),
                    "M" => player2.step(0, 1),
                    "LMeta" => bullets.push(Bullet::new(player1.x, player2.y, Direction::Right)),
                    "Rmeta" => bullets.push(Bullet::new(player1.x, player2.y, Direction::Right)),
                    _ => (),
                }
            }
        }

        player1.draw(display);
        player2.draw(display);
        for bullet in bullets.iter_mut() {
            bullet.fly();
            bullet.draw(display);
        }

        display.flush().unwrap();
        thread::sleep(Duration::from_millis(10));
    }
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
