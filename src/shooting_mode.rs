use crate::interface::{Buttons, Display, Interfaces, Keyboard};
use crate::object::{
    AbsoluteDirection, Hittable, Movable, ObjectEnum, Objects, Player, Status, Team,
};
use crate::object::{DrawableObj, Object};
use device_query::keymap::Keycode;
use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Line, PrimitiveStyleBuilder},
    text::{Alignment, Baseline, Text, TextStyleBuilder},
};
use std::{thread, time::Duration};

pub type Tick = u128;
const TICK_SIZE: Duration = Duration::from_millis(4);
const COUNTDOWN: u16 = 200;
const DAMAGE: i16 = 4;
const DEFAULT_POINT: i16 = 64;

// Shouting Mode
pub fn shooting(interfaces: &mut Interfaces) {
    let mut objects: Objects = Vec::new();
    let mut mono_point: i16 = DEFAULT_POINT;
    let mut di_point: i16 = DEFAULT_POINT;
    let mut count_to_finish: Option<u16> = None;
    let mut winner: Option<Team> = None;
    let mut tick: Tick = 0;
    let character_style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
    let text_style = TextStyleBuilder::new().baseline(Baseline::Top);
    let mono_text_style = text_style.alignment(Alignment::Left).build();
    let di_text_style = text_style.alignment(Alignment::Right).build();
    let life_gauge_stroke = PrimitiveStyleBuilder::new()
        .stroke_color(BinaryColor::On)
        .stroke_width(1)
        .build();

    objects.push(ObjectEnum::Player(player1));
    objects.push(ObjectEnum::Player(player2));

    // Main Loop
    loop {
        // Tick
        tick += 1;
        let status = Status::new();
        let mut new_objects: Objects = Vec::new();

        // Tick
        for object in &mut objects {
            new_objects.append(&mut object.tick(&status));
        }
        objects.append(&mut new_objects);

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

        objects.retain(|o| match o {
            ObjectEnum::Bullet(o) => {
                let pos = o.get_hitbox_position();
                (-10 < pos.x && pos.x < 138) && (-10 < pos.y && pos.y < 74)
            }
            ObjectEnum::Player(_) => true,
        });

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
                        break;
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
