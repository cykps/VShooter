use crate::button::Buttons;
use crate::input_interface::{Display, Inputs};
use crate::key_logger::get_keycodes;
use crate::object::{
    AbsoluteDirection, ObjectEnum, Objects, Player, RelativeDirection, RelativeDirections,
};
use crate::object::{DrawableObj, Object};
use anyhow::Result;
use device_query::{keymap::Keycode, DeviceQuery, DeviceState};
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
use rppal::gpio::{InputPin, Pin};
use rppal::i2c::I2c;
use std::{thread, time::Duration};

const TICK_SIZE: Duration = Duration::from_millis(10);

// Shouting Mode
pub fn shooting(display: &mut Display, buttons: &Buttons) -> Result<()> {
    let mut objects: Objects = Vec::new();

    let player1 = Player::new(
        10,
        32,
        AbsoluteDirection::XPlus,
        vec![Keycode::F],
        vec![Keycode::D],
        vec![Keycode::R],
        vec![Keycode::C],
        0,
    );
    let player2 = Player::new(
        128 - 10,
        32,
        AbsoluteDirection::XMinus,
        vec![Keycode::J],
        vec![Keycode::K],
        vec![Keycode::M],
        vec![Keycode::I],
        1,
    );
    objects.push(ObjectEnum::Player(player1));
    objects.push(ObjectEnum::Player(player2));

    // Main Loop
    loop {
        // Tick
        let keycodes = get_keycodes();
        let button_levels = buttons.get_levels();
        let inputs = Inputs::new(keycodes, button_levels);
        let mut added_objects: Objects = Vec::new();
        for object in &mut objects {
            added_objects.append(&mut object.tick(&inputs));
        }
        objects.append(&mut added_objects);

        // Draw on Display
        display.clear(BinaryColor::Off).unwrap();
        for object in &mut objects {
            object.draw(display);
        }
        display.flush().unwrap();

        thread::sleep(TICK_SIZE);
    }
}
