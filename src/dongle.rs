use std::{
    sync::{Condvar, Mutex},
    time::{Duration, Instant},
};

#[derive(Debug)]
pub struct Dongle {
    pub last_release: Mutex<Instant>,
    pub cv: Condvar,
    pub cooldown: Duration,
}

impl Dongle {
    pub fn new(cooldown: Duration) -> Self {
        Self {
            last_release: Mutex::new(Instant::now()),
            cv: Condvar::new(),
            cooldown,
        }
    }
}
