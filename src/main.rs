use std::{
    env,
    process::exit,
    thread::{sleep, spawn},
    time::{Duration, Instant},
};

#[derive(Clone, Copy, Debug)]
struct Args {
    number_of_coders: u64,
    time_to_burnout: u64,
    time_to_compile: u64,
    time_to_debug: u64,
    time_to_refactor: u64,
    number_of_compiles_required: u64,
    dongle_cooldown: u64,
    scheduler: Scheduler,
}

#[derive(Clone, Copy, Debug)]
enum Scheduler {
    FIFO,
    EDF,
}

#[derive(Debug)]
struct Coder {
    last_compile: u64,
    state: CoderState,
}

#[derive(Clone, Copy, Debug)]
enum CoderState {
    Compiling,
    Debuggin,
    Refactoring,
}

fn parse_args() -> Args {
    let args: Vec<_> = env::args().collect();

    if args.len() != 9 {
        eprintln!("[Error]: wrong number of arguments");
        exit(0);
    }

    let scheduler = args[8].clone();

    let int_args: Vec<u64> = args
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

    let mut thread_handles = vec![];
    let program_start = Instant::now();

    for i in 1..=program_args.number_of_coders {
        thread_handles.push(spawn(move || {
            let mut time = program_start.elapsed().as_millis();
            {
                println!("{time} {i} has taken a dongle");
            }
            time = program_start.elapsed().as_millis();
            println!("{time} {i} is compiling");
            sleep(Duration::from_millis(program_args.time_to_compile));
            time = program_start.elapsed().as_millis();
            println!("{time} {i} is debugging");
            sleep(Duration::from_millis(program_args.time_to_debug));
            time = program_start.elapsed().as_millis();
            println!("{time} {i} is refactoring");
            sleep(Duration::from_millis(program_args.time_to_refactor));
            time = program_start.elapsed().as_millis();
            println!("{time} {i} burned out");
        }));
    }

    thread_handles.into_iter().for_each(|t| {
        let _ = t.join();
    });
}
