use chrono::prelude::*;
use device_query::{keymap::Keycode, DeviceQuery, DeviceState};
use std::{
    sync::{Arc as A, Mutex as M},
    thread,
};

#[derive(Debug, Clone)]
pub struct _Keys {
    pub unix: i64,
    pub delta: i64,
    pub keyboard: Vec<Keycode>,
}
impl _Keys {
    fn new() -> Self {
        Self {
            unix: 0,
            delta: 0,
            keyboard: Vec::new(),
        }
    }
}

type Keys = A<M<_Keys>>;

#[derive(Debug, Clone)]
pub struct KeyLogger {
    keys: Keys,
}

impl KeyLogger {
    pub fn new() -> Self {
        let keys: Keys = A::new(M::new(_Keys::new()));
        let keys_clone = Keys::clone(&keys);
        thread::spawn(move || Self::listener(keys_clone));
        Self { keys }
    }

    pub fn get(&self) -> _Keys {
        self.keys.lock().unwrap().clone()
    }

    fn listener(keys: Keys) {
        let device_state = DeviceState::new();

        let mut prev_key_list = vec![];
        let mut prev_date: DateTime<Local> = Local::now();
        loop {
            let key_list = device_state.get_keys();
            if key_list != prev_key_list {
                let date: DateTime<Local> = Local::now();
                let unix = date.timestamp_millis();
                let delta = date.timestamp_millis() - prev_date.timestamp_millis();
                let _keys = _Keys {
                    unix,
                    delta,
                    keyboard: key_list.clone(),
                };

                *keys.lock().unwrap() = _keys;

                prev_date = date;
                prev_key_list = key_list;
            }

            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
}
