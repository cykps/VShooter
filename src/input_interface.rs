use crate::button::ButtonLevels;
use crate::key_logger::Keycodes;
use rppal::i2c::I2c;
use ssd1306::{mode::BufferedGraphicsMode, prelude::*, Ssd1306};

pub type Display =
    Ssd1306<I2CInterface<I2c>, DisplaySize128x64, BufferedGraphicsMode<DisplaySize128x64>>;

pub struct Inputs {
    pub keycodes: Keycodes,
    pub button_levels: ButtonLevels,
}
impl Inputs {
    pub fn new(keycodes: Keycodes, button_levels: ButtonLevels) -> Self {
        Self {
            keycodes,
            button_levels,
        }
    }
}
