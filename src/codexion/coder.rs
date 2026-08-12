use std::{
    sync::{Arc, Condvar},
    thread::{self, JoinHandle, sleep},
    time::Duration,
};

use crate::args::Args;

pub struct Coder {
    args: Args,
    id: u32,
    start_cond: Arc<Condvar>,
}

impl Coder {
    pub fn new(id: u32, args: Args, start_cond: Arc<Condvar>) -> Self {
        Self {
            args,
            id,
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
