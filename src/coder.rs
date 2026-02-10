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
        // try to acquire left dongle.
        let dongle_left = Arc::clone(&self.dongle_left);
        let coder_number = self.coder_number;
        let left_hand_thread = spawn(move || {
            dongle_left.acquire();
            println!(
                "{:10} {} has taken a dongle",
                program_start.elapsed().as_millis(),
                coder_number
            );
        });

        // try to acquire right dongle.
        let dongle_right = Arc::clone(&self.dongle_right);
        let coder_number = self.coder_number;
        let right_hand_thread = spawn(move || {
            dongle_right.acquire();
            println!(
                "{:10} {} has taken a dongle",
                program_start.elapsed().as_millis(),
                coder_number
            );
        });

        left_hand_thread.join().unwrap();
        right_hand_thread.join().unwrap();
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
