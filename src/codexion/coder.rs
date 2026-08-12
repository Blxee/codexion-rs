use std::{
    sync::{Arc, Condvar, Mutex},
    thread::{self, JoinHandle, sleep},
    time::{Duration, Instant},
};

use crate::{args::Args, codexion::dongle::Dongle, logging::Logging};

pub struct Coder {
    args: Args,
    id: u32,
    last_compile_time: Mutex<Instant>,
    first_dongle: Arc<Dongle>,
    second_dongle: Arc<Dongle>,
    start_signal: Arc<(Mutex<bool>, Condvar)>,
    stop_signal: Arc<(Mutex<bool>, Condvar)>,
    logging: Arc<Logging>,
}

impl Coder {
    pub fn new(
        id: u32,
        args: Args,
        first_dongle: Arc<Dongle>,
        second_dongle: Arc<Dongle>,
        start_signal: Arc<(Mutex<bool>, Condvar)>,
        stop_signal: Arc<(Mutex<bool>, Condvar)>,
        logging: Arc<Logging>,
    ) -> Self {
        Self {
            args,
            id,
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
            let mut start_guard = self.start_signal.0.lock().unwrap();
            while !*start_guard {
                start_guard = self.start_signal.1.wait(start_guard).unwrap();
            }
        }

        // make the latest compile time now
        {
            let mut last_compile_time = self.last_compile_time.lock().unwrap();
            *last_compile_time = Instant::now();
        }

        for _ in 0..self.args.number_of_compiles_required {
            for action in [Coder::compile, Coder::debug, Coder::refactor] {
                if self.should_stop() {
                    break;
                } else {
                    action(self);
                }
            }
        }
    }

    fn compile(&self) {
        // acquire dongles
        let _first_dongle_guard = self.first_dongle.acquire();
        self.logging.acquire(self.id, 1);
        let _second_dongle_guard = self.second_dongle.acquire();
        self.logging.acquire(self.id, 2);

        // compile
        self.logging.compile(self.id);
        self.sleep(self.args.time_to_compile);

        // update latest compile time now
        {
            let mut last_compile_time = self.last_compile_time.lock().unwrap();
            *last_compile_time = Instant::now();
        }

        self.logging.release(self.id, 1);
        self.logging.release(self.id, 2);
    }

    fn debug(&self) {
        self.logging.debug(self.id);
        self.sleep(self.args.time_to_debug);
    }

    fn refactor(&self) {
        self.logging.refactor(self.id);
        self.sleep(self.args.time_to_refactor);
    }

    fn sleep(&self, duration: Duration) {
        let stop_guard = self.stop_signal.0.lock().unwrap();

        let _ = self
            .stop_signal
            .1
            .wait_timeout(stop_guard, duration)
            .unwrap();
    }

    fn should_stop(&self) -> bool {
        *self.stop_signal.0.lock().unwrap()
    }
}
