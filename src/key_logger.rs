use device_query::{keymap::Keycode, DeviceQuery, DeviceState};

pub type Keycodes = Vec<Keycode>;

pub fn get_keycodes() -> Keycodes {
    DeviceState::new().get_keys()
}
