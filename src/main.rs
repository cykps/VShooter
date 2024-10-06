mod button;
mod input_interface;
mod key_logger;
mod object;
mod progress_ring;
mod shooting;

use button::Buttons;
use rppal::{gpio::Gpio, i2c::I2c};
use shooting::shooting;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};
use std::{thread::sleep, time::Duration};

const BUTTON1_PIN: u8 = 16;
const BUTTON2_PIN: u8 = 26;

fn main() {
    // Initialize Button
    let gpio = Gpio::new().unwrap();
    let button1 = gpio.get(BUTTON1_PIN).unwrap().into_input_pullup();
    let button2 = gpio.get(BUTTON2_PIN).unwrap().into_input_pullup();
    let buttons = Buttons::new(button1, button2);
    // Initialize Display
    let i2c = I2c::new().unwrap();
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    loop {
        // Mode: Progress Ring
        progress_ring::run(&mut display).unwrap();

        sleep(Duration::from_millis(500));

        // Mode: Shouting
        shooting(&mut display, &buttons).unwrap();
    }
}
