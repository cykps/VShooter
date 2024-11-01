mod constant;
mod interface;
mod loading_mode;
mod object;
mod shooting_mode;

use embedded_graphics::{pixelcolor::BinaryColor, prelude::Size};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};
use interface::{Buttons, Interfaces, Keyboard, Leds};
use loading_mode::loading_ring;
use rppal::gpio::Gpio;
use shooting_mode::shooting;
use std::{thread::sleep, time::Duration};

fn main() {
    // Initialize display
    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    // Initialize interfaces
    let gpio = Gpio::new().unwrap();
    let mut interfaces = Interfaces::new(
        SimulatorDisplay::<BinaryColor>::new(Size::new(128, 64)),
        Window::new("VShooter", &output_settings),
        Buttons::new(&gpio),
        Leds::new(&gpio),
        Keyboard::new(),
    );

    loop {
        // Mode: Progress Ring
        loading_ring(&mut interfaces.display, &mut interfaces.window);

        // Wait 0.5 sec
        sleep(Duration::from_millis(500));

        // Mode: Shouting
        shooting(&mut interfaces);
    }
}
