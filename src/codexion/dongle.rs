use std::{
    sync::{Arc, Condvar, Mutex},
    time::{Duration, Instant},
};

use crate::args::Args;

pub struct Dongle {
    cooldown: Duration,
    state: Mutex<DongleState>,
    pub release_signal: Condvar,
    stop_signal: Arc<(Mutex<bool>, Condvar)>,
}

enum DongleState {
    Available,
    CoolingDownUntil(Instant),
    Held,
}

pub struct DongleGuard<'a>(&'a Dongle);

impl Dongle {
    pub fn new(args: Args, stop_signal: Arc<(Mutex<bool>, Condvar)>) -> Self {
        Self {
            cooldown: args.dongle_cooldown,
            state: Mutex::new(DongleState::Available),
            release_signal: Condvar::new(),
            stop_signal,
        }
    }

    pub fn acquire<'a>(&'a self) -> Option<DongleGuard<'a>> {
        let mut state = self.state.lock().unwrap();

        loop {
            // check whether a stop signal was sent by the monitor
            if *self.stop_signal.0.lock().unwrap() {
                break None;
            }

            match *state {
                DongleState::Available => {
                    *state = DongleState::Held;
                    break Some(DongleGuard(self));
                }

                DongleState::CoolingDownUntil(next_available) => {
                    let now = Instant::now();

                    if now >= next_available {
                        *state = DongleState::Held;
                        break Some(DongleGuard(self));
                    } else {
                        (state, _) = self
                            .release_signal
                            .wait_timeout(state, next_available - now)
                            .unwrap();
                    }
                }

                DongleState::Held => {
                    state = self.release_signal.wait(state).unwrap();
                }
            }
        }
    }

    pub fn release(&self) {
        let mut state = self.state.lock().unwrap();

        match *state {
            DongleState::Held => {
                *state = DongleState::CoolingDownUntil(Instant::now() + self.cooldown);
                self.release_signal.notify_one();
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
