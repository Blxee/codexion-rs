use std::{
    collections::{BinaryHeap, VecDeque},
    sync::{Arc, Condvar, Mutex},
    time::{Duration, Instant},
};

use crate::{
    args::{Args, Scheduler},
    codexion::Signal,
};

pub struct Dongle {
    cooldown: Duration,
    state: Mutex<DongleState>,
    pub release_cond: Condvar,
    scheduling: Mutex<SchedulingStrategy>,
    stop_signal: Arc<Signal>,
}

enum SchedulingStrategy {
    Queue(VecDeque<u32>),
    Heap(BinaryHeap<u32>),
}

enum DongleState {
    Available,
    CoolingDownUntil(Instant),
    Held,
}

pub struct DongleGuard<'a>(&'a Dongle);

impl Dongle {
    pub fn new(args: Args, stop_signal: Arc<Signal>) -> Self {
        let scheduling = Mutex::new(match args.scheduler {
            Scheduler::FIFO => SchedulingStrategy::Queue(VecDeque::with_capacity(2)),
            Scheduler::EDF => SchedulingStrategy::Heap(BinaryHeap::with_capacity(2)),
        });

        Self {
            cooldown: args.dongle_cooldown,
            state: Mutex::new(DongleState::Available),
            release_cond: Condvar::new(),
            scheduling,
            stop_signal,
        }
    }

    pub fn acquire<'a>(&'a self, coder_id: u32) -> Option<DongleGuard<'a>> {
        let mut state = self.state.lock().unwrap();

        {
            let mut scheduling = self.scheduling.lock().unwrap();
            match &mut *scheduling {
                SchedulingStrategy::Queue(queue) => queue.push_front(coder_id),
                SchedulingStrategy::Heap(heap) => heap.push(coder_id),
            }
        }

        loop {
            // check whether a stop signal was sent by the monitor
            if *self.stop_signal.state.lock().unwrap() {
                break None;
            }

            let result = match *state {
                DongleState::Available => {
                    *state = DongleState::Held;
                    Some(DongleGuard(self))
                }

                DongleState::CoolingDownUntil(next_available) => {
                    let now = Instant::now();

                    if now >= next_available {
                        *state = DongleState::Held;
                        Some(DongleGuard(self))
                    } else {
                        (state, _) = self
                            .release_cond
                            .wait_timeout(state, next_available - now)
                            .unwrap();
                        None
                    }
                }

                DongleState::Held => {
                    state = self.release_cond.wait(state).unwrap();
                    None
                }
            };

            if result.is_some() {
                match &mut *self.scheduling.lock().unwrap() {
                    SchedulingStrategy::Queue(queue) => {
                        let next_id_in_line = queue.back();

                        if let Some(&next_id) = next_id_in_line
                            && next_id == coder_id
                        {
                            queue.pop_back();
                            break result;
                        }
                    }

                    SchedulingStrategy::Heap(heap) => {
                        let next_id_in_line = heap.peek();

                        if let Some(&next_id) = next_id_in_line
                            && next_id == coder_id
                        {
                            heap.pop();
                            break result;
                        }
                    }
                }
            }
        }
    }

    pub fn release(&self) {
        let mut state = self.state.lock().unwrap();

        match *state {
            DongleState::Held => {
                *state = DongleState::CoolingDownUntil(Instant::now() + self.cooldown);
                self.release_cond.notify_one();
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
