use crate::input_interface::{Display, Inputs};
use crate::key_logger::Keycodes;
use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle, Triangle},
};
use rppal::gpio::Level;

// Traits for Objects
pub trait Object {
    fn tick(&mut self, inputs: &Inputs) -> Objects {
        Vec::new()
    }
}

pub trait DrawableObj {
    fn draw(&mut self, display: &mut Display);
}

pub trait Movable {
    fn move_to(&mut self, x: i32, y: i32);
    fn move_by(&mut self, dx: i32, dy: i32);
}

trait MovableRelative {
    fn move_relative(&mut self, forward: i32, left: i32);
}

pub trait Hittable {
    fn get_hitbox_position(&self) -> Position;
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
pub type Objects = Vec<ObjectEnum>;
#[derive(Copy, Clone, PartialEq)]
pub enum Team {
    Mono,
    Di,
}

// Object enum
pub enum ObjectEnum {
    Player(Player),
    Bullet(Bullet),
}
impl ObjectEnum {
    pub fn is_hittable(&self) -> bool {
        match self {
            Self::Player(_) | Self::Bullet(_) => true,
            _ => false,
        }
    }
    pub fn get_team(&self) -> Team {
        match self {
            Self::Player(o) => o.team,
            Self::Bullet(o) => o.team,
        }
    }
}
impl Object for ObjectEnum {
    fn tick(&mut self, inputs: &Inputs) -> Objects {
        match self {
            Self::Player(o) => o.tick(inputs),
            Self::Bullet(o) => o.tick(inputs),
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
impl Hittable for ObjectEnum {
    fn get_hitbox_position(&self) -> Position {
        match self {
            Self::Player(o) => o.get_hitbox_position(),
            Self::Bullet(o) => o.get_hitbox_position(),
        }
    }
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
        }
    }

    fn transfer(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }
}
impl Object for Player {
    fn tick(&mut self, inputs: &Inputs) -> Objects {
        let mut directions = RelativeDirections::new();
        for key in inputs.keycodes.iter() {
            if self.forward_keys.contains(&key) {
                directions.forward = true;
            }
            if self.backward_keys.contains(&key) {
                directions.backward = true;
            }
            if self.left_keys.contains(&key) {
                directions.left = true;
            }
            if self.right_keys.contains(&key) {
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
        if inputs.button_levels[self.shoot_button_idx] == Level::Low {
            vec![ObjectEnum::Bullet(Bullet::new(
                self.x,
                self.y,
                self.direction,
                self.team,
            ))]
        } else {
            Vec::new()
        }
    }
}
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
impl Movable for Player {
    fn move_to(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
    fn move_by(&mut self, dx: i32, dy: i32) {
        self.move_to(self.x + dx, self.y + dy);
    }
}
impl MovableRelative for Player {
    fn move_relative(&mut self, forward: i32, left: i32) {
        let (dx, dy) = match self.direction {
            AbsoluteDirection::XPlus => (forward, -left),
            AbsoluteDirection::XMinus => (-forward, left),
            AbsoluteDirection::YPlus => (left, -forward),
            AbsoluteDirection::YMinus => (-left, forward),
        };
        self.move_by(dx, dy);
    }
}
impl Hittable for Player {
    fn get_hitbox_position(&self) -> Position {
        Position::new(self.x, self.y)
    }
}

// Bullet struct
pub struct Bullet {
    x: i32,
    y: i32,
    direction: AbsoluteDirection,
    team: Team,
    speed: i32,
}
impl Bullet {
    fn new(x: i32, y: i32, direction: AbsoluteDirection, team: Team) -> Self {
        let speed = 1;
        Self {
            x,
            y,
            direction,
            team,
            speed,
        }
    }

    fn transfer(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }

    fn draw(&mut self, display: &mut Display) {
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
}
impl Object for Bullet {
    fn tick(&mut self, inputs: &Inputs) -> Objects {
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

        Vec::new()
    }
}
impl Hittable for Bullet {
    fn get_hitbox_position(&self) -> Position {
        Position::new(self.x, self.y)
    }
}
impl Movable for Bullet {
    fn move_to(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
    fn move_by(&mut self, dx: i32, dy: i32) {
        self.move_to(self.x + dx, self.y + dy);
    }
}
