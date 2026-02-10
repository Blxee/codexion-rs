use std::{
    sync::{Arc, Mutex},
    thread::{sleep, spawn},
    time::{Duration, Instant},
};

use crate::dongle::Dongle;

#[derive(Debug)]
pub struct Coder {
    pub coder_number: u64,
    pub last_compile: Arc<Mutex<Instant>>,
    pub compiles_left: u64,
    pub dongle_left: Arc<Dongle>,
    pub dongle_right: Arc<Dongle>,
    pub time_to_compile: Duration,
    pub time_to_debug: Duration,
    pub time_to_refactor: Duration,
}

impl Coder {
    pub fn new(
        coder_number: u64,
        number_of_compiles_required: u64,
        dongles: [Arc<Dongle>; 2],
        time_to_compile: Duration,
        time_to_debug: Duration,
        time_to_refactor: Duration,
    ) -> Self {
        let [dongle_left, dongle_right] = dongles;
        Self {
            coder_number,
            last_compile: Arc::new(Mutex::new(Instant::now())),
            compiles_left: number_of_compiles_required,
            dongle_left,
            dongle_right,
            time_to_compile,
            time_to_debug,
            time_to_refactor,
        }
    }

    pub fn compile(&mut self, program_start: Instant) {
        // make the even coders try to acquire the left dongle first and vise versa
        let (first, second) = if self.coder_number % 2 == 0 {
            (&self.dongle_left, &self.dongle_right)
        } else {
            (&self.dongle_right, &self.dongle_left)
        };

        // try to acquire the first dongle.
        first.acquire();
        println!(
            "{:10} {} has taken a dongle",
            program_start.elapsed().as_millis(),
            self.coder_number
        );

        // try to acquire second dongle.
        second.acquire();
        println!(
            "{:10} {} has taken a dongle",
            program_start.elapsed().as_millis(),
            self.coder_number
        );

        // Update last compile instant to now.
        {
            let mut last_compile = self.last_compile.lock().unwrap();
            *last_compile = Instant::now();
        }
        let now = program_start.elapsed();
        println!("{:10} {} is compiling", now.as_millis(), self.coder_number);
        sleep(self.time_to_compile);

        // Release both dongles.
        self.dongle_right.release();
        self.dongle_left.release();

        self.compiles_left -= 1;
    }

    pub fn debug(&mut self, program_start: Instant) {
        let now = program_start.elapsed();
        println!("{:10} {} is debugging", now.as_millis(), self.coder_number);
        sleep(self.time_to_debug);
    }

    pub fn refactor(&mut self, program_start: Instant) {
        let now = program_start.elapsed();
        println!(
            "{:10} {} is refactoring",
            now.as_millis(),
            self.coder_number
        );
        sleep(self.time_to_refactor);
    }
}
