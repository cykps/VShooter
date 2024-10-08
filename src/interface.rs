use crate::constant::{BUTTON1_PIN, BUTTON2_PIN, LED1_PIN, LED2_PIN};
use device_query::{keymap::Keycode, DeviceQuery, DeviceState};
use rppal::gpio::Gpio;
use rppal::{
    gpio::{InputPin, Level, OutputPin},
    i2c::I2c,
};
use ssd1306::{mode::BufferedGraphicsMode, prelude::*, Ssd1306};

// Interfaces
pub struct Interfaces {
    pub display: Display,
    pub buttons: Buttons,
    pub leds: Leds,
    pub keyboard: Keyboard,
}
impl Interfaces {
    pub fn new(display: Display, buttons: Buttons, leds: Leds, keyboard: Keyboard) -> Self {
        Self {
            display,
            buttons,
            leds,
            keyboard,
        }
    }
}

// Display
pub type Display =
    Ssd1306<I2CInterface<I2c>, DisplaySize128x64, BufferedGraphicsMode<DisplaySize128x64>>;

// Button
pub struct Buttons {
    button1: InputPin,
    button2: InputPin,
}
impl Buttons {
    pub fn new(gpio: &Gpio) -> Self {
        let button1 = gpio.get(BUTTON1_PIN).unwrap().into_input_pullup();
        let button2 = gpio.get(BUTTON2_PIN).unwrap().into_input_pullup();
        Self { button1, button2 }
    }
    pub fn get_levels(&self) -> ButtonLevels {
        ButtonLevels::new(self.button1.read(), self.button2.read())
    }
}

// Button
pub struct Leds {
    pub led1: OutputPin,
    pub led2: OutputPin,
}
impl Leds {
    pub fn new(gpio: &Gpio) -> Self {
        let led1 = gpio.get(LED1_PIN).unwrap().into_output();
        let led2 = gpio.get(LED2_PIN).unwrap().into_output();
        Self { led1, led2 }
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
