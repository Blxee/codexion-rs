use std::{
    cmp::Reverse,
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
    stop_signal: Arc<Signal>,
}

struct DongleState {
    availability: DongleAvailability,
    scheduling: SchedulingStrategy,
}

enum SchedulingStrategy {
    Queue(VecDeque<u32>),
    Heap(BinaryHeap<(Reverse<Instant>, u32)>),
}

enum DongleAvailability {
    Available,
    CoolingDownUntil(Instant),
    Held,
}

pub struct DongleGuard<'a>(&'a Dongle);

impl Dongle {
    pub fn new(args: Args, stop_signal: Arc<Signal>) -> Self {
        let scheduling = match args.scheduler {
            Scheduler::FIFO => SchedulingStrategy::Queue(VecDeque::with_capacity(2)),
            Scheduler::EDF => SchedulingStrategy::Heap(BinaryHeap::with_capacity(2)),
        };

        Self {
            cooldown: args.dongle_cooldown,
            state: Mutex::new(DongleState {
                availability: DongleAvailability::Available,
                scheduling,
            }),
            release_cond: Condvar::new(),
            stop_signal,
        }
    }

    pub fn acquire<'a>(
        &'a self,
        coder_id: u32,
        last_compile_time: Instant,
    ) -> Option<DongleGuard<'a>> {
        let mut state = self.state.lock().unwrap();

        Self::add_coder_to_waiting_line(&mut state.scheduling, coder_id, last_compile_time);

        loop {
            // check whether a stop signal was sent by the monitor
            if *self.stop_signal.state.lock().unwrap() {
                break None;
            }

            match state.availability {
                // if the dongle is available and the coder is first in line
                // acquire it and pop out of the waiting line
                DongleAvailability::Available => {
                    if Self::try_pop_coder_from_line(&mut state.scheduling, coder_id) {
                        state.availability = DongleAvailability::Held;
                        break Some(DongleGuard(self));
                    }
                }
                // if the dongle is cooling down, wait for the rest of cooldown
                // else, acquire if first in line and pop out
                DongleAvailability::CoolingDownUntil(next_available) => {
                    let now = Instant::now();

                    if now >= next_available {
                        if Self::try_pop_coder_from_line(&mut state.scheduling, coder_id) {
                            state.availability = DongleAvailability::Held;
                            break Some(DongleGuard(self));
                        }
                    } else {
                        (state, _) = self
                            .release_cond
                            .wait_timeout(state, next_available - now)
                            .unwrap();
                    }
                }
                // if the dongle is held, wait for state change
                DongleAvailability::Held => {
                    state = self.release_cond.wait(state).unwrap();
                }
            }
        }
    }

    fn add_coder_to_waiting_line(
        scheduling: &mut SchedulingStrategy,
        coder_id: u32,
        last_compile_time: Instant,
    ) {
        match &mut *scheduling {
            SchedulingStrategy::Queue(queue) => queue.push_front(coder_id),
            SchedulingStrategy::Heap(heap) => heap.push((Reverse(last_compile_time), coder_id)),
        }
    }

    /// Remove coder from waiting line, if he's first
    fn try_pop_coder_from_line(scheduling: &mut SchedulingStrategy, coder_id: u32) -> bool {
        match scheduling {
            SchedulingStrategy::Queue(queue) => {
                let next_id_in_line = queue.back();

                if let Some(&next_id) = next_id_in_line
                    && next_id == coder_id
                {
                    queue.pop_back();
                    return true;
                }
            }

            SchedulingStrategy::Heap(heap) => {
                let next_id_in_line = heap.peek();

                if let Some(&(Reverse(_), next_id)) = next_id_in_line
                    && next_id == coder_id
                {
                    heap.pop();
                    return true;
                }
            }
        }
        false
    }

    /// Change the state to available and notify others who are waiting
    pub fn release(&self) {
        let mut state = self.state.lock().unwrap();

        match state.availability {
            DongleAvailability::Held => {
                state.availability =
                    DongleAvailability::CoolingDownUntil(Instant::now() + self.cooldown);
                self.release_cond.notify_all();
            }
            _ => (),
        }
    }
}

impl<'a> Drop for DongleGuard<'a> {
    /// Release the dongle when the guard drops
    fn drop(&mut self) {
        self.0.release();
    }
}
