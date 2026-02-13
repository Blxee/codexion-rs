use DongleState::*;
use std::{
    collections::VecDeque,
    sync::{Arc, Condvar, Mutex},
    time::{Duration, Instant},
};

#[derive(Debug)]
pub struct Dongle {
    pub cv: Condvar,
    state: Mutex<DongleState>,
    cooldown: Duration,
    shutdown: Arc<Mutex<bool>>,
    priority_queue: Mutex<VecDeque<usize>>,
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
            cv: Condvar::new(),
            state: Mutex::new(CooldownUntil(Instant::now())),
            cooldown,
            shutdown,
            priority_queue: Mutex::new(VecDeque::new()),
        }
    }

    pub fn acquire<'a>(&'a self, coder_number: usize) -> Option<DongleGuard<'a>> {
        let mut guard = self.state.lock().unwrap();

        // add the coder to the waiting queue
        {
            let mut queue = self.priority_queue.lock().unwrap();
            queue.push_back(coder_number);
        }

        loop {
            // if the shutdown signal has been sent by the burnout tracker, exit
            {
                let shutdown = self.shutdown.lock().unwrap();
                if *shutdown {
                    return None;
                }
            }

            let is_my_turn = {
                let queue = self.priority_queue.lock().unwrap();
                *queue.front().unwrap() == coder_number
            };

            if !is_my_turn {
                continue;
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
                        // remove the coder from the queue
                        let mut queue = self.priority_queue.lock().unwrap();
                        queue.pop_front();
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
