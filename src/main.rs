mod args;
mod coder;
mod dongle;

use std::sync::{Arc, Condvar, Mutex};
use std::thread::{sleep, spawn};
use std::time::{Duration, Instant, UNIX_EPOCH};

use crate::args::parse_args;
use crate::coder::Coder;
use crate::dongle::Dongle;

fn foo() {
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

fn coder(dongle: Arc<(Mutex<Instant>, Condvar)>, nb: i32) {
    let (mx, cv) = &*dongle;
    let mut guard = mx.lock().unwrap();
    let mut next_avail = *guard;

    while Instant::now() < next_avail {
        (guard, _) = cv.wait_timeout(guard, next_avail - Instant::now()).unwrap();
        next_avail = *guard;
    }

    *guard = Instant::now() + Duration::from_millis(3000);
    drop(guard);

    println!("coder {nb} is coding..");
    sleep(Duration::from_secs(1));
    println!("coder {nb} finished coding..");
    cv.notify_all();
}

fn main() {
    println!("started..");
    let dongle = Arc::new((
        Mutex::new(Instant::now() - Duration::from_secs(3)),
        Condvar::new(),
    ));

    let coder_dongle = Arc::clone(&dongle);
    let coder1 = spawn(|| coder(coder_dongle, 1));

    let coder_dongle = Arc::clone(&dongle);
    let coder2 = spawn(|| coder(coder_dongle, 2));

    let (mx, cv) = &*dongle;
    let mut guard = mx.lock().unwrap();

    spawn(|| {
        let start = Instant::now();
        for _ in 0..20 {
            println!("time: {:10?}", start.elapsed().as_secs());
            sleep(Duration::from_millis(500));
        }
    });

    for _ in 0..2 {
        guard = cv.wait(guard).unwrap();
        if guard.elapsed() < Duration::from_millis(2000) {
            println!("dongle cooling down..");
        }
    }

    coder1.join().unwrap();
    coder2.join().unwrap();
    println!("finished..");
}
