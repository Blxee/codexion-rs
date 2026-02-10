use std::{
    sync::{Condvar, Mutex},
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

        loop {
            match *guard {
                None => {
                    // if a coder is currently holding the dongle, wait indefinitely.
                    guard = self.cv.wait(guard).unwrap();
                }
                Some(next_available) => {
                    let now = Instant::now();

                    if next_available > now {
                        // wait for the cooldown to end.
                        (guard, _) = self.cv.wait_timeout(guard, next_available - now).unwrap();
                    } else {
                        // make the dongle not available while being held
                        *guard = None;
                        return;
                    }
                }
            }
        }
    }

    pub fn release(&self) {
        let mut guard = self.next_available.lock().unwrap();
        // make the dongle acquirable after cooldown from now.
        *guard = Some(Instant::now() + self.cooldown);
        // notify all the coder that the dongle has been released.
        self.cv.notify_all();
    }
}
