use std::{
    sync::Arc,
    thread::sleep,
    time::{Duration, Instant},
};

use crate::dongle::Dongle;

#[derive(Debug)]
pub struct Coder {
    pub coder_number: u64,
    pub last_compile: Duration,
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
            last_compile: Duration::ZERO,
            compiles_left: number_of_compiles_required,
            dongle_left,
            dongle_right,
            time_to_compile,
            time_to_debug,
            time_to_refactor,
        }
    }

    pub fn compile(&mut self, program_start: Instant) {
        let guard_left = self.dongle_left.acquire();
        println!(
            "{:10} {} has taken a dongle",
            program_start.elapsed().as_millis(),
            self.coder_number
        );

        let guard_right = self.dongle_right.acquire();
        println!(
            "{:10} {} has taken a dongle",
            program_start.elapsed().as_millis(),
            self.coder_number
        );

        let now = program_start.elapsed();
        self.last_compile = now;
        println!("{:10} {} is compiling", now.as_millis(), self.coder_number);
        sleep(self.time_to_compile);

        self.dongle_right.release(guard_left);
        self.dongle_left.release(guard_right);

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
