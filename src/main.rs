mod args;
mod coder;
mod dongle;

use std::process::exit;
use std::sync::{Arc, Mutex};
use std::thread::spawn;
use std::time::{Duration, Instant};

use crate::args::{Args, parse_args};
use crate::coder::Coder;
use crate::dongle::Dongle;

fn coder_thread(mut coder: Coder, program_start: Instant) {
    while coder.compiles_left > 0 {
        coder.compile(program_start);
        coder.debug(program_start);
        coder.refactor(program_start);
    }
}

fn burnout_tracker(
    time_to_burnout: Duration,
    program_start: Instant,
    coders_last_compile: Vec<Arc<Mutex<Instant>>>,
) {
    loop {
        for (i, last_compile) in coders_last_compile.iter().enumerate() {
            let last_compile = last_compile.lock().unwrap();
            if last_compile.elapsed() > time_to_burnout {
                println!(
                    "{:10} {} burned out",
                    program_start.elapsed().as_millis(),
                    i + 1
                );
                exit(0);
            }
        }
    }
}

fn main() {
    let program_args = parse_args();

    let mut thread_handles = vec![];
    let program_start = Instant::now();

    let mut dongles = Vec::new();
    for _ in 0..program_args.number_of_coders {
        dongles.push(Arc::new(Dongle::new(program_args.dongle_cooldown)));
    }

    let mut coders_last_compile = vec![];

    for i in 0..program_args.number_of_coders {
        let left_dongle = if i == 0 {
            dongles.last().unwrap()
        } else {
            &dongles[i as usize - 1]
        };

        let right_dongle = if i as usize == dongles.len() - 1 {
            &dongles[0]
        } else {
            &dongles[i as usize + 1]
        };

        let dongles = [Arc::clone(left_dongle), Arc::clone(right_dongle)];

        let coder = Coder::new(
            i + 1,
            program_args.number_of_compiles_required,
            dongles,
            program_args.time_to_compile,
            program_args.time_to_debug,
            program_args.time_to_refactor,
        );

        coders_last_compile.push(Arc::clone(&coder.last_compile));
        thread_handles.push(spawn(move || coder_thread(coder, program_start)));
    }

    let burnout_handle = spawn(move || {
        burnout_tracker(
            program_args.time_to_burnout,
            program_start,
            coders_last_compile,
        )
    });

    thread_handles.into_iter().for_each(|t| {
        let _ = t.join();
    });
    let _ = burnout_handle.join();
}
