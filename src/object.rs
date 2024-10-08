use crate::constant::{DISPLAY_SIZE_X, EMIT_PROBABILITY, EMIT_TICK_SIZE, LASER_SPAWN_POSITION};
use crate::interface::{ButtonLevels, Display, Interfaces, Keycodes, Led};
use device_query::Keycode;
use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle, Triangle},
};
use rand::Rng;
use rppal::gpio::Level;
use rppal::gpio::Level::*;

pub struct Status {
    keycodes: Keycodes,
    pub button_levels: ButtonLevels,
}
impl Status {
    pub fn new(interfaces: &mut Interfaces) -> Self {
        let keycodes = interfaces.keyboard.get_keycodes();
        let button_levels = interfaces.buttons.get_levels();
        Self {
            keycodes,
            button_levels,
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AbsoluteDirection {
    XPlus,
    XMinus,
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
            vec![Keycode::F],
            vec![Keycode::D],
            vec![Keycode::R],
            vec![Keycode::C],
        );
        let player2 = Player::new(
            128 - 10,
            32,
            AbsoluteDirection::XMinus,
            vec![Keycode::J],
            vec![Keycode::K],
            vec![Keycode::M],
            vec![Keycode::I],
        );
        Self { player1, player2 }
    }
}
// Player struct
pub struct Player {
    x: i32,
    y: i32,
    direction: AbsoluteDirection,
    forward_keys: Keycodes,
    backward_keys: Keycodes,
    left_keys: Keycodes,
    right_keys: Keycodes,
    speed: i32,
    interval: u8,
}
impl Player {
    pub fn new(
        x: i32,
        y: i32,
        direction: AbsoluteDirection,
        forward_keys: Keycodes,
        backward_keys: Keycodes,
        left_keys: Keycodes,
        right_keys: Keycodes,
    ) -> Self {
        let speed = 1;
        Self {
            x,
            y,
            direction,
            forward_keys,
            backward_keys,
            left_keys,
            right_keys,
            speed,
            interval: 0,
        }
    }

    pub fn tick(&mut self, status: &Status) {
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
        };
        self.move_by(dx * 2, dy);
    }
    pub fn get_position(&self) -> Position {
        Position::new(self.x, self.y)
    }
}

// Guns
pub struct Guns {
    pub gun1: Gun,
    pub gun2: Gun,
}
impl Guns {
    pub fn new() -> Self {
        let gun1 = Gun::new(AbsoluteDirection::XPlus);
        let gun2 = Gun::new(AbsoluteDirection::XMinus);
        Self { gun1, gun2 }
    }
}

// Gun
pub struct Gun {
    direction: AbsoluteDirection,
    thread_rng: rand::rngs::ThreadRng,
}
impl Gun {
    pub fn new(direction: AbsoluteDirection) -> Self {
        let thread_rng = rand::thread_rng();
        Self {
            direction,
            thread_rng,
        }
    }
    pub fn shoot(&mut self, player_x: i32) -> Bullet {
        let y = self.thread_rng.gen_range(0..=64);
        Bullet::new(player_x, y, self.direction)
    }
}

// Lasers struct
pub struct Lasers {
    pub laser1: Laser,
    pub laser2: Laser,
}
impl Lasers {
    pub fn new() -> Self {
        let laser1 = Laser::new(AbsoluteDirection::XPlus);
        let laser2 = Laser::new(AbsoluteDirection::XMinus);
        Self { laser1, laser2 }
    }
}

// Laser struct
pub struct Laser {
    direction: AbsoluteDirection,
    thread_rng: rand::rngs::ThreadRng,
    emittable: bool,
    remaining_tick: Option<i32>,
}
impl Laser {
    pub fn new(direction: AbsoluteDirection) -> Self {
        let thread_rng = rand::thread_rng();
        let emittable = false;
        let remaining_tick = None;
        Self {
            direction,
            thread_rng,
            emittable,
            remaining_tick,
        }
    }
    pub fn try_emit(
        &mut self,
        led: &mut Led,
        opponent_player_y: i32,
        button_level: Level,
    ) -> Option<Bullet> {
        match (self.remaining_tick, self.emittable, button_level) {
            (Some(0), _, _) => {
                self.remaining_tick = None;
            }
            (Some(t), _, _) => {
                self.remaining_tick = Some(t - 1);
                return Some(self.emit(opponent_player_y));
            }
            (None, true, Low) => {
                self.remaining_tick = Some(EMIT_TICK_SIZE);
                led.set_low();
                self.emittable = false;
                self.emit(opponent_player_y);
            }
            (None, false, _) => {
                if self.thread_rng.gen_range(0..EMIT_PROBABILITY) == 0 {
                    led.set_high();
                    self.emittable = true;
                }
            }
            _ => (),
        }
        None
    }
    pub fn emit(&mut self, opponent_player_y: i32) -> Bullet {
        match self.direction {
            AbsoluteDirection::XPlus => {
                Bullet::new(-LASER_SPAWN_POSITION, opponent_player_y, self.direction)
            }
            AbsoluteDirection::XMinus => Bullet::new(
                DISPLAY_SIZE_X - LASER_SPAWN_POSITION,
                opponent_player_y,
                self.direction,
            ),
        }
    }
}

// Bullets struct
pub type Bullets = Vec<Bullet>;

// Bullet struct
pub struct Bullet {
    x: i32,
    y: i32,
    direction: AbsoluteDirection,
    speed: i32,
    pub active: bool,
}
impl Bullet {
    fn new(x: i32, y: i32, direction: AbsoluteDirection) -> Self {
        let speed = 1;
        let active = true;
        Self {
            x,
            y,
            direction,
            speed,
            active,
        }
    }

    pub fn tick(&mut self) {
        match self.direction {
            AbsoluteDirection::XPlus => self.transfer(self.speed, 0),
            AbsoluteDirection::XMinus => self.transfer(-self.speed, 0),
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
        }
    }
    pub fn get_position(&self) -> Position {
        Position::new(self.x, self.y)
    }
    pub fn disable(&mut self) {
        self.active = false;
    }
}
