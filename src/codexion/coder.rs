use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use crate::{
    args::Args,
    codexion::{Signal, dongle::Dongle},
    logging::Logging,
};

pub struct Coder {
    args: Args,
    pub id: u32,
    pub compile_count: Mutex<u32>,
    pub last_compile_time: Mutex<Instant>,
    first_dongle: Arc<Dongle>,
    second_dongle: Arc<Dongle>,
    start_signal: Arc<Signal>,
    stop_signal: Arc<Signal>,
    logging: Arc<Logging>,
}

impl Coder {
    pub fn new(
        id: u32,
        args: Args,
        first_dongle: Arc<Dongle>,
        second_dongle: Arc<Dongle>,
        start_signal: Arc<Signal>,
        stop_signal: Arc<Signal>,
        logging: Arc<Logging>,
    ) -> Self {
        Self {
            args,
            id,
            compile_count: Mutex::new(0),
            last_compile_time: Mutex::new(Instant::now()),
            first_dongle,
            second_dongle,
            start_signal,
            stop_signal,
            logging,
        }
    }

    pub fn start_routine(&self) {
        // wait until the main thread signals start
        {
            let mut start_guard = self.start_signal.state.lock().unwrap();
            while !*start_guard {
                start_guard = self.start_signal.cond.wait(start_guard).unwrap();
            }
        }

        // make the latest compile time now
        {
            let mut last_compile_time = self.last_compile_time.lock().unwrap();
            *last_compile_time = Instant::now();
        }

        for _ in 0..self.args.number_of_compiles_required {
            for action in [Coder::compile, Coder::debug, Coder::refactor] {
                action(self);

                let should_stop = *self.stop_signal.state.lock().unwrap();
                if should_stop {
                    return;
                }
            }
        }
    }

    fn compile(&self) {
        {
            // acquire first dongle
            let first_dongle_guard = self
                .first_dongle
                .acquire(self.id, self.get_last_compile_time());
            if first_dongle_guard.is_none() {
                return;
            }
            self.logging.acquire(self.id, self.first_dongle.id);
            // acquire second dongle
            let second_dongle_guard = self
                .second_dongle
                .acquire(self.id, self.get_last_compile_time());
            if second_dongle_guard.is_none() {
                return;
            }
            self.logging.acquire(self.id, self.second_dongle.id);

            // compile
            self.logging.compile(self.id);
            let timedout = self.sleep(self.args.time_to_compile);

            if !timedout {
                return;
            }

            self.logging.release(self.id, self.first_dongle.id);
            self.logging.release(self.id, self.second_dongle.id);
        }

        // update latest compile time to now
        {
            let mut last_compile_time = self.last_compile_time.lock().unwrap();
            *last_compile_time = Instant::now();
        }
        // update compile count
        {
            let mut compile_count = self.compile_count.lock().unwrap();
            *compile_count += 1;
        }
    }

    fn debug(&self) {
        self.logging.debug(self.id);
        self.sleep(self.args.time_to_debug);
    }

    fn refactor(&self) {
        self.logging.refactor(self.id);
        self.sleep(self.args.time_to_refactor);
    }

    fn sleep(&self, duration: Duration) -> bool {
        let stop_guard = self.stop_signal.state.lock().unwrap();

        let (_guard, timeout) = self
            .stop_signal
            .cond
            .wait_timeout(stop_guard, duration)
            .unwrap();

        timeout.timed_out()
    }

    fn get_last_compile_time(&self) -> Instant {
        *self.last_compile_time.lock().unwrap()
    }
}
