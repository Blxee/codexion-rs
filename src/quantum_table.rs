use std::{
    sync::{Arc, Mutex},
    thread::spawn,
    time::{Duration, Instant},
};

use crate::{args::Args, coder::Coder, dongle::Dongle};

pub struct QuantumTable {
    coders: Vec<Coder>,
    dongles: Vec<Arc<Dongle>>,
    coders_last_compiles: Vec<Arc<Mutex<Instant>>>,
    time_to_burnout: Duration,
    shutdown: Arc<Mutex<bool>>,
    program_start: Instant,
}

impl QuantumTable {
    pub fn new(program_args: Args) -> Self {
        let mut coders = Vec::with_capacity(program_args.number_of_coders);
        let shutdown = Arc::new(Mutex::new(false));
        let dongles = Self::create_dongles(
            program_args.number_of_coders,
            program_args.dongle_cooldown,
            &shutdown,
        );
        let mut coders_last_compiles = Vec::with_capacity(program_args.number_of_coders);
        let program_start = Instant::now();

        for i in 0..program_args.number_of_coders {
            let coder_dongles = Self::get_coder_dongles(i, &dongles);

            let coder = Coder::new(
                i + 1,
                program_args.number_of_compiles_required,
                coder_dongles,
                program_args.time_to_compile,
                program_args.time_to_debug,
                program_args.time_to_refactor,
            );

            coders_last_compiles.push(Arc::clone(&coder.last_compile));
            coders.push(coder);
        }

        Self {
            coders,
            dongles,
            coders_last_compiles,
            time_to_burnout: program_args.time_to_burnout,
            shutdown,
            program_start,
        }
    }

    /// create as much dongles as there are coders.
    fn create_dongles(
        amount: usize,
        dongle_cooldown: Duration,
        shutdown: &Arc<Mutex<bool>>,
    ) -> Vec<Arc<Dongle>> {
        let mut dongles = Vec::with_capacity(amount);
        for _ in 0..amount {
            dongles.push(Arc::new(Dongle::new(dongle_cooldown, Arc::clone(shutdown))));
        }
        dongles
    }

    /// get left and right dongles for a coder given his index.
    fn get_coder_dongles(coder_idx: usize, dongles: &Vec<Arc<Dongle>>) -> [Arc<Dongle>; 2] {
        let left_dongle = if coder_idx == 0 {
            dongles.last().unwrap()
        } else {
            &dongles[coder_idx - 1]
        };

        let right_dongle = if coder_idx == dongles.len() - 1 {
            &dongles[0]
        } else {
            &dongles[coder_idx + 1]
        };

        [Arc::clone(left_dongle), Arc::clone(right_dongle)]
    }

    /// if the shutdown signal has been sent by the burnout tracker, exit
    fn should_shutdown(shutdown: &Arc<Mutex<bool>>) -> bool {
        let shutdown = shutdown.lock().unwrap();
        *shutdown
    }

    fn coder_thread(mut coder: Coder, program_start: Instant, shutdown: Arc<Mutex<bool>>) {
        while coder.compiles_left > 0 {
            if Self::should_shutdown(&shutdown) {
                return;
            }
            coder.compile(program_start);

            if Self::should_shutdown(&shutdown) {
                return;
            }
            coder.debug(program_start);

            if Self::should_shutdown(&shutdown) {
                return;
            }
            coder.refactor(program_start);
        }
    }

    fn burnout_tracker(
        dongles: Vec<Arc<Dongle>>,
        coders_last_compiles: Vec<Arc<Mutex<Instant>>>,
        time_to_burnout: Duration,
        shutdown: Arc<Mutex<bool>>,
        program_start: Instant,
    ) {
        loop {
            for (i, last_compile) in coders_last_compiles.iter().enumerate() {
                let last_compile = last_compile.lock().unwrap();
                // if this coder has burned out
                if last_compile.elapsed() > time_to_burnout {
                    println!(
                        "{:10} \x1b[31m{} burned out\x1b[0m\t\t😩",
                        program_start.elapsed().as_millis(),
                        i + 1
                    );
                    // change the shutdown flag to true
                    {
                        let mut guard = shutdown.lock().unwrap();
                        *guard = true;
                    }
                    // notify all coders
                    for dongle in dongles {
                        dongle.cv.notify_all();
                    }
                    return;
                }
            }
        }
    }

    pub fn start_coding(self) {
        let mut thread_handles = Vec::new();

        let Self {
            coders,
            dongles,
            coders_last_compiles,
            time_to_burnout,
            shutdown,
            program_start,
        } = self;

        for coder in coders {
            let shutdown = Arc::clone(&shutdown);
            thread_handles.push(spawn(move || {
                Self::coder_thread(coder, self.program_start, shutdown)
            }));
        }

        thread_handles.push(spawn(move || {
            Self::burnout_tracker(
                dongles,
                coders_last_compiles,
                time_to_burnout,
                shutdown,
                program_start,
            )
        }));

        for handle in thread_handles {
            handle.join().unwrap();
        }
    }
}
