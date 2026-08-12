mod coder;
mod dongle;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Instant;

use crate::args::Args;
use crate::codexion::coder::Coder;
use crate::codexion::dongle::Dongle;
use crate::logging::Logging;

pub struct Codexion {
    args: Args,
    coders: Vec<Coder>,
    start_signal: Arc<(Mutex<bool>, Condvar)>,
    logging: Arc<Logging>,
}

impl Codexion {
    pub fn new(args: Args) -> Self {
        let start_signal = Arc::new((Mutex::new(false), Condvar::new()));
        let logging = Arc::new(Logging::new());

        let dongles: Vec<Arc<Dongle>> = (0..args.number_of_coders)
            .map(|_| Arc::new(Dongle::new(args)))
            .collect();

        let mut coders = Vec::new();

        for i in 0..args.number_of_coders {
            let mut first_idx = i as usize;
            let mut second_idx = ((i + 1) % args.number_of_coders) as usize;

            if i % 2 == 0 {
                (first_idx, second_idx) = (second_idx, first_idx);
            }

            let first_dongle = Arc::clone(&dongles[first_idx]);
            let second_dongle = Arc::clone(&dongles[second_idx]);

            let coder = Coder::new(
                i + 1,
                args,
                first_dongle,
                second_dongle,
                Arc::clone(&start_signal),
                Arc::clone(&logging),
            );
            coders.push(coder);
        }

        Self {
            args,
            coders,
            start_signal,
            logging,
        }
    }

    pub fn start(self) {
        // create all the threads
        let mut handles = Vec::new();
        for coder in self.coders {
            let handle = thread::spawn(move || coder.start_routine());
            handles.push(handle);
        }
        // set start time to this instant for logging
        {
            let mut logging_start_time = self.logging.start_time_lock.lock().unwrap();
            *logging_start_time = Instant::now();
        }
        // signal the coders to start
        {
            let mut start_mutex = self.start_signal.0.lock().unwrap();
            *start_mutex = true;
            self.start_signal.1.notify_all();
        }
        // join all threads
        for handle in handles {
            handle.join().unwrap();
        }
    }
}
