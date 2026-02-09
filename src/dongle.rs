use std::{
    sync::{Condvar, Mutex},
    time::{Duration, Instant},
};

#[derive(Debug)]
pub struct Dongle {
    pub next_available: Mutex<Instant>,
    pub cv: Condvar,
    pub cooldown: Duration,
}

impl Dongle {
    pub fn new(cooldown: Duration) -> Self {
        Self {
            next_available: Mutex::new(Instant::now()),
            cv: Condvar::new(),
            cooldown,
        }
    }

    pub fn acquire(&self) {
        let mut guard = self.next_available.lock().unwrap();
        let mut cooldown_left = *guard - Instant::now();

        while !cooldown_left.is_zero() {
            (guard, _) = self.cv.wait_timeout(guard, cooldown_left).unwrap();
            cooldown_left = *guard - Instant::now();
        }
    }

    pub fn release(&self) {
        let mut guard = self.next_available.lock().unwrap();
        *guard = Instant::now() + self.cooldown;
    }
}
