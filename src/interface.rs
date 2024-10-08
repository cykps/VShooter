use device_query::{keymap::Keycode, DeviceQuery, DeviceState};
use rppal::{
    gpio::{InputPin, Level},
    i2c::I2c,
};
use ssd1306::{mode::BufferedGraphicsMode, prelude::*, Ssd1306};

// Display
pub type Display =
    Ssd1306<I2CInterface<I2c>, DisplaySize128x64, BufferedGraphicsMode<DisplaySize128x64>>;

// Button
pub struct Buttons {
    pub button1: InputPin,
    pub button2: InputPin,
}
impl Buttons {
    pub fn new(button1: InputPin, button2: InputPin) -> Self {
        Self { button1, button2 }
    }
    pub fn get_levels(&self) -> ButtonLevels {
        ButtonLevels::new(self.button1.read(), self.button2.read())
    }
}

pub struct ButtonLevels {
    button1_level: Level,
    button2_level: Level,
}
impl ButtonLevels {
    pub fn new(button1_level: Level, button2_level: Level) -> Self {
        Self {
            button1_level,
            button2_level,
        }
    }
}

// Keyboard
pub type Keycodes = Vec<Keycode>;
pub struct Keyboard {
    device_state: DeviceState,
}
impl Keyboard {
    pub fn new() -> Self {
        Self {
            device_state: DeviceState::new(),
        }
    }
    pub fn get_keycodes(&self) -> Keycodes {
        self.device_state.get_keys()
    }
}
