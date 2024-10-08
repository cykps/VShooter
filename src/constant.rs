use std::time::Duration;

// Pin
pub const BUTTON1_PIN: u8 = 16;
pub const BUTTON2_PIN: u8 = 26;
pub const LED1_PIN: u8 = 19;
pub const LED2_PIN: u8 = 20;

// Tick
pub const TICK_SIZE: Duration = Duration::from_millis(4);

// Hitpoint
pub const INITIAL_HITPOINT: i8 = 64;
pub const BULLET_DAMEGE: i8 = 4;

// Display
pub const DISPLAY_SIZE_X: i32 = 128;
pub const DISPLAY_SIZE_Y: i32 = 64;

// Hit
pub const HIT_DISTANCE: i32 = 1;
pub const DISPLAY_MARGIN: i32 = 8;

// Exit cool time
pub const RESULT_TICK_SIZE: i32 = 100;

// Shoot
pub const SHOOT_INTERVAL: u128 = 1000;

// Laser
pub const EMIT_TICK_SIZE: i32 = 20;
pub const LASER_SPAWN_POSITION: i32 = 9;
pub const EMIT_PROBABILITY: i32 = 280;
