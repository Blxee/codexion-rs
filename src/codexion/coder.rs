use std::{
    sync::{Arc, Condvar, Mutex},
    thread::{self, JoinHandle, sleep},
    time::{Duration, Instant},
};

use crate::{args::Args, codexion::dongle::Dongle, logging::Logging};

pub struct Coder {
    args: Args,
    id: u32,
    first_dongle: Arc<Dongle>,
    second_dongle: Arc<Dongle>,
    start_signal: Arc<(Mutex<bool>, Condvar)>,
    logging: Arc<Logging>,
}

impl Coder {
    pub fn new(
        id: u32,
        args: Args,
        first_dongle: Arc<Dongle>,
        second_dongle: Arc<Dongle>,
        start_signal: Arc<(Mutex<bool>, Condvar)>,
        logging: Arc<Logging>,
    ) -> Self {
        Self {
            args,
            id,
            first_dongle,
            second_dongle,
            start_signal,
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

        for _ in 0..self.args.number_of_compiles_required {
            self.compile();
            self.debug();
            self.refactor();
        }
    }

    fn compile(&self) {
        let _first_dongle_guard = self.first_dongle.acquire();
        self.logging.acquire(self.id, 1);
        let _second_dongle_guard = self.second_dongle.acquire();
        self.logging.acquire(self.id, 2);

        self.logging.compile(self.id);
        sleep(self.args.time_to_compile);

        self.logging.release(self.id, 1);
        self.logging.release(self.id, 2);
    }

    fn debug(&self) {
        self.logging.debug(self.id);
        sleep(self.args.time_to_debug);
    }

    fn refactor(&self) {
        self.logging.refactor(self.id);
        sleep(self.args.time_to_refactor);
    }
}
