use std::{
    env,
    process::exit,
    thread::{sleep, spawn},
    time::{Duration, Instant},
};

#[derive(Clone, Copy, Debug)]
struct Args {
    number_of_coders: u32,
    time_to_burnout: u32,
    time_to_compile: u32,
    time_to_debug: u32,
    time_to_refactor: u32,
    number_of_compiles_required: u32,
    dongle_cooldown: u32,
    scheduler: Scheduler,
}

#[derive(Clone, Copy, Debug)]
enum Scheduler {
    FIFO,
    EDF,
}

fn parse_args() -> Args {
    let args: Vec<_> = env::args().collect();

    if args.len() != 9 {
        eprintln!("[Error]: wrong number of arguments");
        exit(0);
    }

    let scheduler = args[8].clone();

    let int_args: Vec<u32> = args
        .into_iter()
        .skip(1)
        .take(7)
        .map_while(|arg| arg.parse().ok())
        .collect();

    let [
        number_of_coders,
        time_to_burnout,
        time_to_compile,
        time_to_debug,
        time_to_refactor,
        number_of_compiles_required,
        dongle_cooldown,
    ] = int_args[..]
    else {
        panic!("[Error]: could not parse arguments")
    };

    let scheduler = match scheduler.as_str() {
        "fifo" => Scheduler::FIFO,
        "edf" => Scheduler::EDF,
        _ => panic!("[Error]: invalid scheduler value"),
    };

    Args {
        number_of_coders,
        time_to_burnout,
        time_to_compile,
        time_to_debug,
        time_to_refactor,
        number_of_compiles_required,
        dongle_cooldown,
        scheduler,
    }
}

fn main() {
    let program_args = parse_args();
    println!("args: {program_args:?}");
    let mut thread_handles = vec![];
    let program_start = Instant::now();

    for i in 0..5 {
        thread_handles.push(spawn(move || {
            sleep(Duration::from_millis(10 * i));
            let time = program_start.elapsed().as_millis();
            println!("hello from thread {i}: {time}");
        }));
    }

    thread_handles.into_iter().for_each(|t| {
        let _ = t.join();
    });
}
