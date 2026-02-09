use std::{env, path::Path, process::exit, time::Duration};

#[derive(Clone, Copy, Debug)]
pub struct Args {
    pub number_of_coders: u64,
    pub time_to_burnout: Duration,
    pub time_to_compile: Duration,
    pub time_to_debug: Duration,
    pub time_to_refactor: Duration,
    pub number_of_compiles_required: u64,
    pub dongle_cooldown: Duration,
    pub scheduler: Scheduler,
}

#[derive(Clone, Copy, Debug)]
enum Scheduler {
    FIFO,
    EDF,
}

pub fn parse_args() -> Args {
    let args: Vec<_> = env::args().collect();

    if args.len() != 9 {
        eprintln!("[Error]: wrong number of arguments");
        println!(
            "\
{} <number_of_coders> <time_to_burnout> <time_to_compile> <time_to_debug> <time_to_refactor> <number_of_compiles_required> <dongle_cooldown> <scheduler>\
            ",
            args[0].split('/').last().unwrap()
        );
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

    let time_to_burnout = Duration::from_millis(time_to_burnout);
    let time_to_compile = Duration::from_millis(time_to_compile);
    let time_to_debug = Duration::from_millis(time_to_debug);
    let time_to_refactor = Duration::from_millis(time_to_refactor);
    let dongle_cooldown = Duration::from_millis(dongle_cooldown);

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
