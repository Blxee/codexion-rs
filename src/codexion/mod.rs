mod coder;
mod dongle;
use std::sync::{Arc, Condvar};
use std::thread;

use crate::args::Args;
use crate::codexion::coder::Coder;
use crate::codexion::dongle::Dongle;

pub struct Codexion {
    args: Args,
    coders: Vec<Coder>,
    start_cond: Arc<Condvar>,
}

impl Codexion {
    pub fn new(args: Args) -> Self {
        let start_cond = Arc::new(Condvar::new());
        let coders = (0..args.number_of_coders)
            .map(|id| {
                Coder::new(
                    id,
                    args,
                    [Arc::new(Dongle::new()), Arc::new(Dongle::new())],
                    Arc::clone(&start_cond),
                )
            })
            .collect();
        Self {
            args,
            coders,
            start_cond,
        }
    }

    pub fn start(self) {
        let mut handles = Vec::new();

        for coder in self.coders {
            let handle = thread::spawn(move || coder.routine());
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}
