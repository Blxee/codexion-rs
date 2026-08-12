mod coder;
mod dongle;
use std::sync::{Arc, Condvar};
use std::thread;
use std::time::Instant;

use crate::args::Args;
use crate::codexion::coder::Coder;
use crate::codexion::dongle::Dongle;
use crate::logging::Logging;

pub struct Codexion {
    args: Args,
    coders: Vec<Coder>,
    start_cond: Arc<Condvar>,
    logging: Arc<Logging>,
}

impl Codexion {
    pub fn new(args: Args) -> Self {
        let start_cond = Arc::new(Condvar::new());
        let logging = Arc::new(Logging::new());

        let coders = (0..args.number_of_coders)
            .map(|id| {
                Coder::new(
                    id + 1,
                    args,
                    [Arc::new(Dongle::new()), Arc::new(Dongle::new())],
                    Arc::clone(&start_cond),
                    Arc::clone(&logging),
                )
            })
            .collect();
        Self {
            args,
            coders,
            start_cond,
            logging,
        }
    }

    pub fn start(self) {
        let mut handles = Vec::new();

        let start_time = Instant::now();

        for coder in self.coders {
            let handle = thread::spawn(move || coder.start_routine(start_time));
            handles.push(handle);
        }

        {
            let mut logging_start_time = self.logging.start_time_lock.lock().unwrap();
            *logging_start_time = Instant::now();
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}
