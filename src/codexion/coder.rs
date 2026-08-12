use std::{
    sync::{Arc, Condvar, Mutex},
    thread::{self, JoinHandle, sleep},
    time::{Duration, Instant},
};

use crate::{args::Args, codexion::dongle::Dongle, logging::Logging};

pub struct Coder {
    args: Args,
    id: u32,
    dongles: [Arc<Dongle>; 2],
    start_cond: Arc<Condvar>,
    logging: Arc<Logging>,
}

impl Coder {
    pub fn new(
        id: u32,
        args: Args,
        dongles: [Arc<Dongle>; 2],
        start_cond: Arc<Condvar>,
        logging: Arc<Logging>,
    ) -> Self {
        Self {
            args,
            id,
            dongles,
            start_cond,
            logging,
        }
    }

    pub fn start_routine(&self, start_time: Instant) {
        for _ in 0..self.args.number_of_compiles_required {
            self.compile();
            self.debug();
            self.refactor();
        }
    }

    fn compile(&self) {
        let mut handles = Vec::new();

        for (i, dongle) in self.dongles.iter().enumerate() {
            handles.push(dongle.acquire());
            self.logging.acquire(self.id, i as u32 + 1);
        }

        self.logging.compile(self.id);
        sleep(self.args.time_to_compile);

        for i in 0..handles.len() {
            self.logging.release(self.id, i as u32 + 1);
        }
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
