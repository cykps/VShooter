use crate::interface::{ButtonLevels, Display, Interfaces, Keycodes};
use crate::shooting_mode::Tick;
use device_query::Keycode;
use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle, Triangle},
};
use rand::Rng;
use rppal::gpio::Level;

const SHOOT_INTERVAL: u128 = 1000;

pub struct Status {
    keycodes: Keycodes,
    button_levels: ButtonLevels,
    tick: Tick,
    mono_life: u8,
    di_life: u8,
}
impl Status {
    pub fn new(interfaces: &mut Interfaces, tick: Tick, mono_life: u8, di_life: u8) -> Self {
        let keycodes = interfaces.keyboard.get_keycodes();
        let button_levels = interfaces.buttons.get_levels();
        Self {
            keycodes,
            button_levels,
            tick,
            mono_life,
            di_life,
        }
    }
}

#[derive(Debug)]
pub struct RelativeDirections {
    forward: bool,
    backward: bool,
    left: bool,
    right: bool,
}
impl RelativeDirections {
    fn new() -> Self {
        Self {
            forward: false,
            backward: false,
            left: false,
            right: false,
        }
    }
}

#[derive(Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}
impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

// Objects type
#[derive(Copy, Clone, PartialEq)]
pub enum Team {
    Mono,
    Di,
}

// Absolute and relative direction enum
#[derive(Debug, PartialEq)]
pub enum RelativeDirection {
    Forward,
    Backward,
    Left,
    Right,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AbsoluteDirection {
    XPlus,
    XMinus,
    YPlus,
    YMinus,
}

// Players struct
pub struct Players {
    pub player1: Player,
    pub player2: Player,
}
impl Players {
    pub fn new() -> Self {
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
        Self { player1, player2 }
    }
}
// Player struct
pub struct Player {
    x: i32,
    y: i32,
    direction: AbsoluteDirection,
    team: Team,
    forward_keys: Keycodes,
    backward_keys: Keycodes,
    left_keys: Keycodes,
    right_keys: Keycodes,
    shoot_button_idx: usize,
    speed: i32,
    interval: u8,
    bom: bool,
    bom_time: u8,
}
impl Player {
    pub fn new(
        x: i32,
        y: i32,
        direction: AbsoluteDirection,
        team: Team,
        forward_keys: Keycodes,
        backward_keys: Keycodes,
        left_keys: Keycodes,
        right_keys: Keycodes,
        shoot_button_idx: usize,
    ) -> Self {
        let speed = 1;
        Self {
            x,
            y,
            team,
            direction,
            forward_keys,
            backward_keys,
            left_keys,
            right_keys,
            shoot_button_idx,
            speed,
            interval: 0,
            bom: false,
            bom_time: 0,
        }
    }

    pub fn tick(&mut self, status: &Status) -> Bullets {
        let mut directions = RelativeDirections::new();
        for key in status.keycodes.iter() {
            if self.forward_keys.contains(key) {
                directions.forward = true;
            }
            if self.backward_keys.contains(key) {
                directions.backward = true;
            }
            if self.left_keys.contains(key) {
                directions.left = true;
            }
            if self.right_keys.contains(key) {
                directions.right = true;
            }
        }
        let forward = match (directions.forward, directions.backward) {
            (true, false) => self.speed,
            (false, true) => -self.speed,
            _ => 0,
        };
        let left = match (directions.left, directions.right) {
            (true, false) => self.speed,
            (false, true) => -self.speed,
            _ => 0,
        };
        self.move_relative(forward, left);
        if self.interval != 0 {
            self.interval -= 1;
        }
        let mut rng = rand::thread_rng();
        let mut adds = Vec::new();
        if rng.gen_range(0..200) == 0 {
            self.bom = true;
        }
        if self.bom && status.button_levels.button1_level == Level::Low {
            self.bom_time = 30;
        }
        if self.bom_time > 0 {
            self.bom = false;
            self.bom_time -= 1;
            adds.push(Bullet::new(
                {
                    if self.direction == AbsoluteDirection::XPlus {
                        128
                    } else {
                        0
                    }
                },
                self.y,
                {
                    if self.direction == AbsoluteDirection::XPlus {
                        AbsoluteDirection::XMinus
                    } else {
                        AbsoluteDirection::XPlus
                    }
                },
                {
                    if self.team == Team::Mono {
                        Team::Di
                    } else {
                        Team::Mono
                    }
                },
            ));
        }
        if self.interval == 0 {
            self.interval = (SHOOT_INTERVAL / (status.tick / 2 + 100)) as u8;
            adds.push(Bullet::new(
                self.x,
                rng.gen_range(0..=64),
                self.direction,
                self.team,
            ));
        }
        adds
    }

    pub fn draw(&mut self, display: &mut Display) {
        let style = PrimitiveStyleBuilder::new()
            .fill_color(BinaryColor::On)
            .build();

        Rectangle::new(Point::new(self.x - 3, self.y - 3), Size::new(7, 7))
            .into_styled(style)
            .draw(display)
            .unwrap();
    }

    fn move_to(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
    fn move_by(&mut self, dx: i32, dy: i32) {
        let new_pos = Position::new(self.x + dx, self.y + dy);
        let new_dx = if !(0 <= new_pos.x && new_pos.x <= 128) {
            0
        } else {
            dx
        };
        let new_dy = if !(0 <= new_pos.y && new_pos.y <= 64) {
            0
        } else {
            dy
        };
        self.move_to(self.x + new_dx, self.y + new_dy);
    }
    fn move_relative(&mut self, forward: i32, left: i32) {
        let (dx, dy) = match self.direction {
            AbsoluteDirection::XPlus => (if self.x + forward < 50 { forward } else { 0 }, -left),
            AbsoluteDirection::XMinus => (if self.x - forward > 78 { -forward } else { 0 }, left),
            AbsoluteDirection::YPlus => (left, -forward),
            AbsoluteDirection::YMinus => (-left, forward),
        };
        self.move_by(dx * 2, dy);
    }
    pub fn get_position(&self) -> Position {
        Position::new(self.x, self.y)
    }
}

// Bullets struct
pub type Bullets = Vec<Bullet>;

// Bullet struct
pub struct Bullet {
    x: i32,
    y: i32,
    direction: AbsoluteDirection,
    team: Team,
    speed: i32,
    pub active: bool,
}
impl Bullet {
    fn new(x: i32, y: i32, direction: AbsoluteDirection, team: Team) -> Self {
        let speed = 1;
        let active = true;
        Self {
            x,
            y,
            direction,
            team,
            speed,
            active,
        }
    }

    pub fn tick(&mut self) {
        match self.direction {
            AbsoluteDirection::XPlus => self.transfer(self.speed, 0),
            AbsoluteDirection::XMinus => self.transfer(-self.speed, 0),
            AbsoluteDirection::YPlus => self.transfer(0, self.speed),
            AbsoluteDirection::YMinus => self.transfer(0, -self.speed),
        }
        if self.direction == AbsoluteDirection::XPlus {
            self.transfer(self.speed, 0);
        } else if self.direction == AbsoluteDirection::XMinus {
            self.transfer(-self.speed, 0);
        }
    }

    fn transfer(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }

    pub fn draw(&mut self, display: &mut Display) {
        let style = PrimitiveStyleBuilder::new()
            .fill_color(BinaryColor::On)
            .build();
        match self.direction {
            AbsoluteDirection::XPlus => {
                Triangle::new(
                    Point::new(self.x + 2, self.y),
                    Point::new(self.x - 2, self.y + 1),
                    Point::new(self.x - 2, self.y - 1),
                )
                .into_styled(style)
                .draw(display)
                .unwrap();
            }
            AbsoluteDirection::XMinus => {
                Triangle::new(
                    Point::new(self.x - 2, self.y),
                    Point::new(self.x + 2, self.y + 1),
                    Point::new(self.x + 2, self.y - 1),
                )
                .into_styled(style)
                .draw(display)
                .unwrap();
            }
            AbsoluteDirection::YPlus => {
                Triangle::new(
                    Point::new(self.x, self.y + 2),
                    Point::new(self.x + 1, self.y - 2),
                    Point::new(self.x - 1, self.y - 2),
                )
                .into_styled(style)
                .draw(display)
                .unwrap();
            }
            AbsoluteDirection::YMinus => {
                Triangle::new(
                    Point::new(self.x, self.y - 2),
                    Point::new(self.x + 1, self.y + 2),
                    Point::new(self.x - 1, self.y + 2),
                )
                .into_styled(style)
                .draw(display)
                .unwrap();
            }
        }
    }
    pub fn get_position(&self) -> Position {
        Position::new(self.x, self.y)
    }
    pub fn move_to(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
    fn move_by(&mut self, dx: i32, dy: i32) {
        self.move_to(self.x + dx, self.y + dy);
    }
    pub fn disable(&mut self) {
        self.active = false;
    }
}
