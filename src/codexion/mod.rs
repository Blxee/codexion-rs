mod coder;
mod dongle;
use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, sleep};
use std::time::Instant;

use crate::args::Args;
use crate::codexion::coder::Coder;
use crate::codexion::dongle::Dongle;
use crate::logging::Logging;

pub struct Codexion {
    args: Args,
    dongles: Vec<Arc<Dongle>>,
    coders: Vec<Arc<Coder>>,
    start_signal: Arc<Signal>,
    stop_signal: Arc<Signal>,
    logging: Arc<Logging>,
}

struct Signal {
    state: Mutex<bool>,
    cond: Condvar,
}

impl Codexion {
    pub fn new(args: Args) -> Self {
        let start_signal = Arc::new(Signal {
            state: Mutex::new(false),
            cond: Condvar::new(),
        });
        let stop_signal = Arc::new(Signal {
            state: Mutex::new(false),
            cond: Condvar::new(),
        });

        let logging = Arc::new(Logging::new());

        let dongles: Vec<Arc<Dongle>> = (0..args.number_of_coders)
            .map(|_| Arc::new(Dongle::new(args, Arc::clone(&stop_signal))))
            .collect();

        let mut coders = Vec::new();
        // create coders
        for i in 0..args.number_of_coders {
            let mut first_idx = i as usize;
            let mut second_idx = ((i + 1) % args.number_of_coders) as usize;

            if first_idx > second_idx {
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
                Arc::clone(&stop_signal),
                Arc::clone(&logging),
            );
            coders.push(Arc::new(coder));
        }

        Self {
            args,
            dongles,
            coders,
            start_signal,
            stop_signal,
            logging,
        }
    }

    pub fn start(self) {
        // create all the threads
        let mut handles = Vec::new();

        for coder in &self.coders {
            let coder = Arc::clone(&coder);

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
            let mut start_mutex = self.start_signal.state.lock().unwrap();
            *start_mutex = true;
            self.start_signal.cond.notify_all();
        }
        // start monitoring coders
        self.monitor();
        // join all threads
        for handle in handles {
            handle.join().unwrap();
        }
    }

    fn monitor(&self) {
        loop {
            let mut all_finished = true;
            let mut earliest_compile_time = Instant::now();

            for coder in &self.coders {
                let compile_count = *coder.compile_count.lock().unwrap();
                // if coder has reached mandatory compiles, skip him
                if compile_count == self.args.number_of_compiles_required {
                    continue;
                } else {
                    all_finished = false;
                }

                let last_compile_time = *coder.last_compile_time.lock().unwrap();
                if last_compile_time < earliest_compile_time {
                    earliest_compile_time = last_compile_time;
                }

                // if last compile time is more than burnout time
                // stop the simulation
                if Instant::now() - last_compile_time >= self.args.time_to_burnout {
                    self.shutdown();
                    self.logging.burnout(coder.id);
                    return;
                }
            }

            if all_finished {
                break;
            }

            let elapsed = Instant::now() - earliest_compile_time;
            sleep(self.args.time_to_burnout.saturating_sub(elapsed));
        }
    }

    fn shutdown(&self) {
        let mut stop = self.stop_signal.state.lock().unwrap();
        *stop = true;
        self.stop_signal.cond.notify_all();

        for dongle in &self.dongles {
            dongle.release_cond.notify_all();
        }
    }
}
