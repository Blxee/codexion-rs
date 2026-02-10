use std::{
    sync::{Condvar, Mutex, MutexGuard},
    time::{Duration, Instant},
};

#[derive(Debug)]
pub struct Dongle {
    pub next_available: Mutex<Option<Instant>>,
    pub cv: Condvar,
    pub cooldown: Duration,
}

impl Dongle {
    pub fn new(cooldown: Duration) -> Self {
        Self {
            next_available: Mutex::new(Some(Instant::now())),
            cv: Condvar::new(),
            cooldown,
        }
    }

    pub fn acquire(&self) {
        let mut guard = self.next_available.lock().unwrap();

        // if a coder is currently holding the dongle, wait indefinitely.
        while guard.is_none() {
            guard = self.cv.wait(guard).unwrap();
        }

        // wait for the cooldown to end.
        let next_available = guard.unwrap();
        while next_available > Instant::now() {
            (guard, _) = self
                .cv
                .wait_timeout(guard, next_available - Instant::now())
                .unwrap();
        }

        // make the dongle not available while being held
        *guard = None;
    }

    pub fn release(&self) {
        let mut guard = self.next_available.lock().unwrap();
        // make the dongle acquirable after cooldown from now.
        *guard = Some(Instant::now() + self.cooldown);
        // notify all the coder that the dongle has been released.
        self.cv.notify_all();
    }
}
