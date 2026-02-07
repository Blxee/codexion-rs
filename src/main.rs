mod args;
mod coder;
mod dongle;

use std::sync::Arc;
use std::thread::spawn;
use std::time::Instant;

use crate::args::parse_args;
use crate::coder::Coder;
use crate::dongle::Dongle;

fn main() {
    let program_args = parse_args();

    let mut thread_handles = vec![];
    let program_start = Instant::now();

    let mut dongles = Vec::new();
    for _ in 0..program_args.number_of_coders {
        dongles.push(Arc::new(Dongle::new(program_args.dongle_cooldown)));
    }

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

        thread_handles.push(spawn(move || {
            let mut coder = Coder::new(
                i + 1,
                program_args.number_of_compiles_required,
                dongles,
                program_args.time_to_compile,
                program_args.time_to_debug,
                program_args.time_to_refactor,
            );

            while coder.compiles_left > 0 {
                coder.compile(program_start);
                coder.debug(program_start);
                coder.refactor(program_start);
            }
        }));
    }

    println!("{} {} burned out", 0, 0);
    thread_handles.into_iter().for_each(|t| {
        let _ = t.join();
    });
}
