use DongleState::*;
use std::{
    sync::{Arc, Condvar, Mutex},
    time::{Duration, Instant},
};

#[derive(Debug)]
pub struct Dongle {
    state: Mutex<DongleState>,
    pub cv: Condvar,
    pub cooldown: Duration,
    shutdown: Arc<Mutex<bool>>,
}

#[derive(Debug)]
enum DongleState {
    Held,
    CooldownUntil(Instant),
}

pub struct DongleGuard<'a> {
    dongle: &'a Dongle,
}

impl<'a> Drop for DongleGuard<'a> {
    fn drop(&mut self) {
        self.dongle.release();
    }
}

impl Dongle {
    pub fn new(cooldown: Duration, shutdown: Arc<Mutex<bool>>) -> Self {
        Self {
            state: Mutex::new(CooldownUntil(Instant::now())),
            cv: Condvar::new(),
            cooldown,
            shutdown,
        }
    }

    pub fn acquire<'a>(&'a self) -> Option<DongleGuard<'a>> {
        let mut guard = self.state.lock().unwrap();

        loop {
            // if the shutdown signal has been sent by the burnout tracker, exit
            {
                let shutdown = self.shutdown.lock().unwrap();
                if *shutdown {
                    return None;
                }
            }

            match *guard {
                Held => {
                    // if a coder is currently holding the dongle, wait indefinitely.
                    guard = self.cv.wait(guard).unwrap();
                }
                CooldownUntil(next_available) => {
                    let now = Instant::now();

                    if next_available > now {
                        // wait for the cooldown to end.
                        (guard, _) = self.cv.wait_timeout(guard, next_available - now).unwrap();
                    } else {
                        // make the dongle not available while being held
                        *guard = Held;
                        return Some(DongleGuard { dongle: self });
                    }
                }
            }
        }
    }

    pub fn release(&self) {
        let mut guard = self.state.lock().unwrap();
        // make the dongle acquirable after cooldown from now.
        *guard = CooldownUntil(Instant::now() + self.cooldown);
        // notify all the coder that the dongle has been released.
        self.cv.notify_all();
    }
}
