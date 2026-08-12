use std::{
    sync::{Arc, Condvar, Mutex},
    thread::{self, JoinHandle, sleep},
    time::Duration,
};

use crate::{args::Args, codexion::dongle::Dongle};

pub struct Coder {
    args: Args,
    id: u32,
    dongles: [Arc<Dongle>; 2],
    start_cond: Arc<Condvar>,
}

impl Coder {
    pub fn new(id: u32, args: Args, dongles: [Arc<Dongle>; 2], start_cond: Arc<Condvar>) -> Self {
        Self {
            args,
            id,
            dongles,
            start_cond,
        }
    }

    pub fn routine(&self) {
        println!("starting routine");

        for _ in 0..self.args.number_of_compiles_required {
            self.compile();
            self.debug();
            self.refactor();
        }
    }

    fn compile(&self) {
        let mut handles = Vec::new();
        for dongle in &self.dongles {
            handles.push(dongle.acquire());
        }
        println!("compiling..");
        sleep(Duration::from_millis(1000));
    }
    fn debug(&self) {
        println!("debugging..");
        sleep(Duration::from_millis(1000));
    }
    fn refactor(&self) {
        println!("refactoring..");
        sleep(Duration::from_millis(1000));
    }
}
