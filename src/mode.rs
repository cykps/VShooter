use crate::button::Buttons;
use crate::input_interface::{Display, Inputs};
use crate::key_logger::get_keycodes;
use crate::object::{
    AbsoluteDirection, Hittable, Movable, ObjectEnum, Objects, Player, RelativeDirection,
    RelativeDirections, Team,
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
use itertools::Itertools;
use rppal::gpio::{InputPin, Pin};
use rppal::i2c::I2c;
use std::{thread, time::Duration};

const TICK_SIZE: Duration = Duration::from_millis(10);

// Shouting Mode
pub fn shooting(display: &mut Display, buttons: &Buttons) -> Result<()> {
    let mut objects: Objects = Vec::new();
    let mut mono_point: i8 = 128;
    let mut di_point: i8 = 128;

    let player1 = Player::new(
        10,
        32,
        AbsoluteDirection::XPlus,
        Team::Mono,
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
        Team::Di,
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
        let inputs = Inputs::new(get_keycodes(), buttons.get_levels());
        let mut add_objects: Objects = Vec::new();
        for object in &mut objects {
            add_objects.append(&mut object.tick(&inputs));
        }
        objects.append(&mut add_objects);

        // Hit
        let hittable: Vec<&mut ObjectEnum> =
            objects.iter_mut().filter(|x| x.is_hittable()).collect();
        let mut mono: Vec<&mut ObjectEnum> = Vec::new();
        let mut di: Vec<&mut ObjectEnum> = Vec::new();
        for obj in hittable {
            match obj.get_team() {
                Team::Mono => mono.push(obj),
                Team::Di => di.push(obj),
            }
        }
        for m in &*mono {
            for d in &*di {
                let m_pos = m.get_hitbox_position();
                let d_pos = d.get_hitbox_position();
                if (m_pos.x - d_pos.x).abs() + (m_pos.y - d_pos.y).abs() == 0 {
                    match (&m, &d) {
                        (&&&mut ObjectEnum::Bullet(&m_row), &&&mut ObjectEnum::Bullet(&d_row)) => {
                            (m_row).move_to(-100, -100);
                            (d_row).move_to(-200, -200);
                        }
                        (ObjectEnum::Player(_), ObjectEnum::Bullet(d)) => {
                            mono_point -= 4;
                            d.move_to(-200, -200);
                        }
                        (ObjectEnum::Bullet(m), ObjectEnum::Player(_)) => {
                            m.move_to(-100, -100);
                            di_point -= 4;
                        }
                        (ObjectEnum::Player(_), ObjectEnum::Player(_)) => (),
                    }
                }
            }
        }

        // Draw on Display
        display.clear(BinaryColor::Off).unwrap();
        for object in &mut objects {
            object.draw(display);
        }
        display.flush().unwrap();

        thread::sleep(TICK_SIZE);
    }
}
