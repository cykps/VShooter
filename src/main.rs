mod constant;
mod interface;
mod loading_mode;
mod object;
mod shooting_mode;

use interface::{Buttons, Interfaces, Keyboard, Leds};
use loading_mode::loading_ring;
use rppal::{gpio::Gpio, i2c::I2c};
use shooting_mode::shooting;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};
use std::{thread::sleep, time::Duration};

fn main() {
    // Initialize display
    let i2c = I2c::new().unwrap();
    let i2c_interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(i2c_interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();
    // Initialize interfaces
    let gpio = Gpio::new().unwrap();
    let mut interfaces = Interfaces::new(
        display,
        Buttons::new(&gpio),
        Leds::new(&gpio),
        Keyboard::new(),
    );

    loop {
        // Mode: Progress Ring
        loading_ring(&mut interfaces.display);

        // Wait 0.5 sec
        sleep(Duration::from_millis(500));

        // Mode: Shouting
        shooting(&mut interfaces);
    }
}
