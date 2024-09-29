use rppal::gpio::{InputPin, Level};

pub type ButtonLevels = Vec<Level>;

pub struct Buttons {
    pub button1: InputPin,
    pub button2: InputPin,
}
impl Buttons {
    pub fn new(button1: InputPin, button2: InputPin) -> Self {
        Self { button1, button2 }
    }
    pub fn get_levels(&self) -> ButtonLevels {
        vec![self.button1.read(), self.button2.read()]
    }
}
