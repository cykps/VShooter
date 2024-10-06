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
    primitives::{Arc, Line, PrimitiveStyleBuilder, Rectangle, StrokeAlignment, Triangle},
    text::{Alignment, Baseline, Text, TextStyleBuilder},
    transform::Transform,
};
use itertools::Itertools;
use rppal::gpio::{InputPin, Pin};
use rppal::i2c::I2c;
use std::{thread, time::Duration};

const TICK_SIZE: Duration = Duration::from_millis(4);
const COUNTDOWN: u16 = 200;
const DAMAGE: i16 = 4;

// Shouting Mode
pub fn shooting(display: &mut Display, buttons: &Buttons) -> Result<()> {
    let mut objects: Objects = Vec::new();
    let mut mono_point: i16 = 64;
    let mut di_point: i16 = 64;
    let mut countdown: Option<u16> = None;
    let mut mono_win: Option<bool> = None;
    let mut tick: u128 = 0;
    let mono_text_style = TextStyleBuilder::new()
        .baseline(Baseline::Top)
        .alignment(Alignment::Left)
        .build();
    let di_text_style = TextStyleBuilder::new()
        .baseline(Baseline::Top)
        .alignment(Alignment::Right)
        .build();
    let character_style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
    let line_stroke = PrimitiveStyleBuilder::new()
        .stroke_color(BinaryColor::On)
        .stroke_width(1)
        .build();

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
        tick += 1;
        let inputs = Inputs::new(get_keycodes(), buttons.get_levels(), tick);
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
        for m in mono.iter_mut() {
            for d in di.iter_mut() {
                let m_ref = &mut **m;
                let d_ref = &mut **d;

                let m_pos = m_ref.get_hitbox_position();
                let d_pos = d_ref.get_hitbox_position();
                if (m_pos.x - d_pos.x).abs() + (m_pos.y - d_pos.y).abs() <= 1 {
                    match (m_ref, d_ref) {
                        (ObjectEnum::Bullet(ref mut m), ObjectEnum::Bullet(ref mut d)) => {
                            m.move_to(-100, -100);
                            d.move_to(-200, -200);
                        }
                        (ObjectEnum::Player(_), ObjectEnum::Bullet(ref mut d)) => {
                            mono_point -= DAMAGE;
                            d.move_to(-200, -200);
                        }
                        (ObjectEnum::Bullet(ref mut m), ObjectEnum::Player(_)) => {
                            m.move_to(-100, -100);
                            di_point -= DAMAGE;
                        }
                        (ObjectEnum::Player(_), ObjectEnum::Player(_)) => (),
                    }
                }
            }
        }

        //for object in objects {
        //    let pos = object.get_hitbox_position();
        //    if !(-10 < pos.x < 138) || !(-10 < pos.y < 74) {
        //
        //    }
        //}

        objects = objects
            .into_iter()
            .filter(|o| match o {
                ObjectEnum::Bullet(o) => {
                    let pos = o.get_hitbox_position();
                    ((-10 < pos.x && pos.x < 138) && (-10 < pos.y && pos.y < 74))
                }
                ObjectEnum::Player(o) => true,
            })
            .collect();

        // Draw on Display
        display.clear(BinaryColor::Off).unwrap();
        for object in &mut objects {
            object.draw(display);
        }
        // Draw Point Bar
        Line::new(Point::new(0, 0), Point::new(mono_point.into(), 0))
            .into_styled(line_stroke)
            .draw(display)
            .unwrap();
        Line::new(Point::new((128 - di_point).into(), 0), Point::new(128, 0))
            .into_styled(line_stroke)
            .draw(display)
            .unwrap();

        if mono_point <= 0 || di_point <= 0 {
            if mono_win.is_none() {
                mono_win = Some(di_point <= 0);
            }

            let mono_text: &str;
            let di_text: &str;
            (mono_text, di_text) = if mono_win.unwrap() {
                ("Win", "Lose")
            } else {
                ("Lose", "Win")
            };
            Text::with_text_style(mono_text, Point::zero(), character_style, mono_text_style)
                .draw(display)
                .unwrap();
            Text::with_text_style(di_text, Point::new(128, 0), character_style, di_text_style)
                .draw(display)
                .unwrap();

            match countdown {
                Some(c) => {
                    if c == 0 {
                        break Ok(());
                    } else {
                        countdown = Some(c - 1)
                    }
                }
                None => countdown = Some(COUNTDOWN),
            }
        }
        display.flush().unwrap();

        thread::sleep(TICK_SIZE);
    }
}
