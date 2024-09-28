use crate::key_logger::{KeyLogger, _Keys};
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
use rppal::i2c::I2c;
use ssd1306::{mode::BufferedGraphicsMode, prelude::*, Ssd1306};
use std::{thread, time::Duration};

const tick_SIZE: Duration = Duration::from_millis(10);

// Shouting Mode
pub fn shouting(
    display: &mut Ssd1306<
        I2CInterface<I2c>,
        DisplaySize128x64,
        BufferedGraphicsMode<DisplaySize128x64>,
    >,
    key_logger: &mut KeyLogger,
) -> Result<()> {
    let mut objects: Objects = Vec::new();

    let mut player1 = Player::new(
        10,
        32,
        vec![Keycode::J],
        vec![Keycode::K],
        vec![Keycode::M],
        vec![Keycode::I],
    );
    let mut player2 = Player::new(
        128 - 10,
        32,
        vec![Keycode::F],
        vec![Keycode::D],
        vec![Keycode::R],
        vec![Keycode::C],
    );
    objects.push(ObjectEnum::Player(player1));
    objects.push(ObjectEnum::Player(player2));

    // Main Loop
    loop {
        display.clear(BinaryColor::Off).unwrap();
        //let key_list = key_logger.get().keyboard;
        //
        //for key in key_list {
        //    println!("{:?}", key);
        //    match key.as_str() {
        //        "F" => player1.step(1, 0),
        //        "D" => player1.step(-1, 0),
        //        "C" => player1.step(0, 1),
        //        "R" => player1.step(0, -1),
        //        "J" => player2.step(-1, 0),
        //        "K" => player2.step(1, 0),
        //        "I" => player2.step(0, -1),
        //        "M" => player2.step(0, 1),
        //        "LMeta" => objects.push(ObjectEnum::Bullet(Bullet::new(
        //            player1.x,
        //            player1.y,
        //            Direction::Right,
        //        ))),
        //        "RMeta" => objects.push(ObjectEnum::Bullet(Bullet::new(
        //            player2.x,
        //            player2.y,
        //            Direction::Left,
        //        ))),
        //        _ => (),
        //    }
        //}
        let key_list = key_logger.get().keyboard;

        // Keybind
        for object in &mut objects {
            object.receive_key_bind(&key_list);
        }

        // Tick
        for object in &mut objects {
            object.tick();
        }

        // Draw
        for object in &mut objects {
            object.draw(display);
        }

        display.flush().unwrap();
        thread::sleep(tick_SIZE);
    }
}

type Display =
    Ssd1306<I2CInterface<I2c>, DisplaySize128x64, BufferedGraphicsMode<DisplaySize128x64>>;
type Objects = Vec<ObjectEnum>;
type Keycodes = Vec<Keycode>;

// Object Trait
trait Object {
    fn tick(&mut self) {}
}

trait DrawableObj {
    fn draw(&mut self, display: &mut Display) {}
}

trait ReceivableKeyBindObj {
    fn receive_key_bind(&mut self, keys: &Keycodes) {}
}

enum ObjectEnum {
    Player(Player),
    Bullet(Bullet),
}
impl Object for ObjectEnum {
    fn tick(&mut self) {
        match self {
            Self::Player(o) => o.tick(),
            Self::Bullet(o) => o.tick(),
        }
    }
}
impl DrawableObj for ObjectEnum {
    fn draw(&mut self, display: &mut Display) {
        match self {
            Self::Player(o) => o.draw(display),
            Self::Bullet(o) => o.draw(display),
        }
    }
}
impl ReceivableKeyBindObj for ObjectEnum {
    fn receive_key_bind(&mut self, keys: &Keycodes) {
        match self {
            Self::Player(o) => o.receive_key_bind(keys),
            _ => (),
        }
    }
}

// Player
struct Player {
    x: i32,
    y: i32,
    step: i32,
    front_key: Vec<Keycode>,
    back_key: Vec<Keycode>,
    left_key: Vec<Keycode>,
    right_key: Vec<Keycode>,
}
impl Player {
    fn new(
        x: i32,
        y: i32,
        front_key: Vec<Keycode>,
        back_key: Vec<Keycode>,
        left_key: Vec<Keycode>,
        right_key: Vec<Keycode>,
    ) -> Self {
        let step = 2;
        Self {
            x,
            y,
            step,
            front_key,
            back_key,
            left_key,
            right_key,
        }
    }

    fn transfer(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }

    fn step(&mut self, x: i32, y: i32) {
        self.transfer(x * self.step, y * self.step);
    }
}
impl Object for Player {}
impl DrawableObj for Player {
    fn draw(&mut self, display: &mut Display) {
        let style = PrimitiveStyleBuilder::new()
            .fill_color(BinaryColor::On)
            .build();

        Rectangle::new(Point::new(self.x - 2, self.y - 2), Size::new(5, 5))
            .into_styled(style)
            .draw(display)
            .unwrap();
    }
}
impl ReceivableKeyBindObj for Player {
    fn receive_key_bind(&mut self, keys: &Keycodes) {
        let mut step_direction = StepDirection::new();
        for key in keys.iter() {
            if self.front_key.contains(key) {
                step_direction.front = true;
            }
            if self.back_key.contains(key) {
                step_direction.front = true;
            }
            if self.left_key.contains(key) {
                step_direction.front = true;
            }
            if self.right_key.contains(key) {
                step_direction.front = true;
            }
        }
    }
}
struct StepDirection {
    front: bool,
    back: bool,
    left: bool,
    right: bool,
}
impl StepDirection {
    fn new() -> Self {
        Self {
            front: false,
            back: false,
            left: false,
            right: false,
        }
    }
}

// Bullet
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

impl Object for Bullet {
    fn tick(&mut self) {
        if self.direction == Direction::Right {
            self.transfer(self.speed, 0);
        } else {
            self.transfer(-self.speed, 0);
        }
    }
}

#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
}
