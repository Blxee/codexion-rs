use std::{
    sync::{Condvar, Mutex, MutexGuard},
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

    pub fn acquire(&self) -> MutexGuard<'_, Instant> {
        let mut guard = self.next_available.lock().unwrap();
        let mut cooldown_left = *guard - Instant::now();

        while !cooldown_left.is_zero() {
            (guard, _) = self.cv.wait_timeout(guard, cooldown_left).unwrap();
            cooldown_left = *guard - Instant::now();
        }
        guard
    }

    pub fn release(&self, mut guard: MutexGuard<'_, Instant>) {
        *guard = Instant::now() + self.cooldown;
        self.cv.notify_all();
    }
}
