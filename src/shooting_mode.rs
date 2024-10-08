use crate::constant::{
    BULLET_DAMEGE, DISPLAY_MARGIN, DISPLAY_SIZE_X, DISPLAY_SIZE_Y, HIT_DISTANCE, INITIAL_HITPOINT,
    TICK_FOR_SHOWING_RESULT, TICK_SIZE,
};
use crate::interface::{Buttons, Display, Interfaces, Keyboard};
use crate::object::{Bullets, Players, Status, Team};
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

// Shouting Mode
pub fn shooting(interfaces: &mut Interfaces) {
    let mut mono_hitpoint = INITIAL_HITPOINT;
    let mut di_hitpoint = INITIAL_HITPOINT;
    let mut tick: Tick = 0;
    let mut winner: Option<Team> = None;
    let mut mono_result_text: Option<&str> = None;
    let mut di_result_text: Option<&str> = None;
    let mut tick_for_exit: Option<i32> = None;

    let character_style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
    let text_style = TextStyleBuilder::new().baseline(Baseline::Top);
    let mono_text_style = text_style.alignment(Alignment::Left).build();
    let di_text_style = text_style.alignment(Alignment::Right).build();
    let hitpoint_bar_stroke = PrimitiveStyleBuilder::new()
        .stroke_color(BinaryColor::On)
        .stroke_width(1)
        .build();

    let mut players = Players::new();
    let mut bullets_mono = Vec::new();
    let mut bullets_di = Vec::new();

    // Main Loop
    loop {
        // Tick
        tick += 1;
        let status = Status::new(interfaces, tick, mono_hitpoint, di_hitpoint);

        // Process par tick
        // player
        bullets_mono.append(&mut players.player1.tick(&status));
        bullets_di.append(&mut players.player2.tick(&status));
        // bullets
        for bullet in &mut bullets_mono {
            bullet.tick();
        }
        for bullet in &mut bullets_di {
            bullet.tick();
        }

        // Hit
        // player and bullet
        if winner.is_none() {
            let p_pos = players.player1.get_position();
            for b in bullets_di.iter_mut() {
                let b_pos = b.get_position();
                if (p_pos.x - b_pos.x).abs() + (p_pos.y - b_pos.y).abs() <= HIT_DISTANCE {
                    mono_hitpoint -= BULLET_DAMEGE;
                    b.disable();
                }
            }
            let p_pos = players.player2.get_position();
            for b in bullets_mono.iter_mut() {
                let b_pos = b.get_position();
                if (p_pos.x - b_pos.x).abs() + (p_pos.y - b_pos.y).abs() <= HIT_DISTANCE {
                    di_hitpoint -= BULLET_DAMEGE;
                    b.disable();
                }
            }
            // bullet and bullet
            for m in bullets_mono.iter_mut() {
                for d in bullets_di.iter_mut() {
                    if m.active && d.active {
                        let m_pos = m.get_position();
                        let d_pos = d.get_position();
                        if (m_pos.x - d_pos.x).abs() + (m_pos.y - d_pos.y).abs() <= HIT_DISTANCE {
                            m.disable();
                            d.disable();
                        }
                    }
                }
            }
        }

        // Remove bullets at outside of display
        bullets_mono.retain(|b| {
            b.active || {
                let pos = b.get_position();
                (-DISPLAY_MARGIN < pos.x && pos.x < DISPLAY_SIZE_X + DISPLAY_MARGIN)
                    && (-DISPLAY_MARGIN < pos.y && pos.y < DISPLAY_SIZE_Y + DISPLAY_MARGIN)
            }
        });
        bullets_di.retain(|b| {
            b.active || {
                let pos = b.get_position();
                (-DISPLAY_MARGIN < pos.x && pos.x < DISPLAY_SIZE_X + DISPLAY_MARGIN)
                    && (-DISPLAY_MARGIN < pos.y && pos.y < DISPLAY_SIZE_Y + DISPLAY_MARGIN)
            }
        });

        // Draw on display
        // clear display
        interfaces.display.clear(BinaryColor::Off).unwrap();

        // draw players
        players.player1.draw(&mut interfaces.display);
        players.player2.draw(&mut interfaces.display);

        // draw bullets
        for bullet in &mut bullets_mono {
            if bullet.active {
                bullet.draw(&mut interfaces.display);
            }
        }
        for bullet in &mut bullets_di {
            if bullet.active {
                bullet.draw(&mut interfaces.display);
            }
        }

        // Draw hitpoint bar
        Line::new(Point::new(0, 0), Point::new(mono_hitpoint.into(), 0))
            .into_styled(hitpoint_bar_stroke)
            .draw(&mut interfaces.display)
            .unwrap();
        Line::new(
            Point::new(DISPLAY_SIZE_X - (di_hitpoint as i32), 0),
            Point::new(DISPLAY_SIZE_X, 0),
        )
        .into_styled(hitpoint_bar_stroke)
        .draw(&mut interfaces.display)
        .unwrap();

        // Decide winner
        match (winner, mono_hitpoint, di_hitpoint) {
            (None, mono_hp, di_hp) if (mono_hp == 0 || di_hp == 0) && mono_hp > di_hp => {
                (mono_result_text, di_result_text) = (Some("Win"), Some("Lose"));
                winner = Some(Team::Mono);
            }
            (None, mono_hp, di_hp) if (mono_hp == 0 || di_hp == 0) && di_hp > mono_hp => {
                (mono_result_text, di_result_text) = (Some("Lose"), Some("Win"));
                winner = Some(Team::Di);
            }
            (None, mono_hp, di_hp) if (mono_hp == 0 || di_hp == 0) && mono_hp == di_hp => {
                mono_hitpoint = BULLET_DAMEGE;
                mono_hitpoint = BULLET_DAMEGE;
            }
            _ => (),
        }

        // Draw result
        if winner.is_some() {
            Text::with_text_style(
                mono_result_text.unwrap(),
                Point::zero(),
                character_style,
                mono_text_style,
            )
            .draw(&mut interfaces.display)
            .unwrap();
            Text::with_text_style(
                di_result_text.unwrap(),
                Point::new(DISPLAY_SIZE_X, 0),
                character_style,
                di_text_style,
            )
            .draw(&mut interfaces.display)
            .unwrap();
        }

        // Flush display
        interfaces.display.flush().unwrap();

        // Exit
        match tick_for_exit {
            Some(0) => {
                break;
            }
            Some(t) => {
                tick_for_exit = Some(t - 1);
            }
            None => {
                if winner.is_some() {
                    tick_for_exit = Some(TICK_FOR_SHOWING_RESULT);
                }
            }
        }

        // Sleep
        thread::sleep(TICK_SIZE);
    }
}