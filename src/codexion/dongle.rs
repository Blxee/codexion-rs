use std::{
    sync::{Condvar, Mutex},
    time::{Duration, Instant},
};

use crate::args::Args;

pub struct Dongle {
    cooldown: Duration,
    state: Mutex<DongleState>,
    release_signal: Condvar,
}

enum DongleState {
    Available,
    CoolingDownUntil(Instant),
    Held,
}

pub struct DongleGuard<'a>(&'a Dongle);

impl Dongle {
    pub fn new(args: Args) -> Self {
        Self {
            cooldown: args.dongle_cooldown,
            state: Mutex::new(DongleState::Available),
            release_signal: Condvar::new(),
        }
    }

    pub fn acquire<'a>(&'a self) -> DongleGuard<'a> {
        let mut state = self.state.lock().unwrap();

        loop {
            match *state {
                DongleState::Available => {
                    *state = DongleState::Held;
                    break DongleGuard(self);
                }

                DongleState::CoolingDownUntil(next_available) => {
                    if Instant::now() >= next_available {
                        *state = DongleState::Held;
                        break DongleGuard(self);
                    } else {
                        (state, _) = self
                            .release_signal
                            .wait_timeout(state, next_available - Instant::now())
                            .unwrap();
                    }
                }

                DongleState::Held => {
                    state = self.release_signal.wait(state).unwrap();
                }
            }
        }
    }

    fn release(&self) {
        let mut state = self.state.lock().unwrap();

        match *state {
            DongleState::Held => {
                *state = DongleState::CoolingDownUntil(Instant::now() + self.cooldown);
                self.release_signal.notify_all();
            }
            _ => (),
        }
    }
}

impl<'a> Drop for DongleGuard<'a> {
    fn drop(&mut self) {
        self.0.release();
    }
}
