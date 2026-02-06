use std::{
    env,
    process::exit,
    sync::{Arc, Condvar, Mutex},
    thread::{sleep, spawn},
    time::{Duration, Instant},
};

#[derive(Clone, Copy, Debug)]
struct Args {
    number_of_coders: u64,
    time_to_burnout: Duration,
    time_to_compile: Duration,
    time_to_debug: Duration,
    time_to_refactor: Duration,
    number_of_compiles_required: u64,
    dongle_cooldown: Duration,
    scheduler: Scheduler,
}

#[derive(Clone, Copy, Debug)]
enum Scheduler {
    FIFO,
    EDF,
}

type Dongle = Arc<(Mutex<Duration>, Condvar)>;

#[derive(Debug)]
struct Coder {
    coder_number: u64,
    last_compile: Duration,
    compiles_left: u64,
    dongle1: Dongle,
    dongle2: Dongle,
    time_to_compile: Duration,
    time_to_debug: Duration,
    time_to_refactor: Duration,
}

impl Coder {
    fn new(
        coder_number: u64,
        number_of_compiles_required: u64,
        dongles: [Dongle; 2],
        time_to_compile: Duration,
        time_to_debug: Duration,
        time_to_refactor: Duration,
    ) -> Self {
        let [dongle1, dongle2] = dongles;
        Self {
            coder_number,
            last_compile: Duration::ZERO,
            compiles_left: number_of_compiles_required,
            dongle1,
            dongle2,
            time_to_compile,
            time_to_debug,
            time_to_refactor,
        }
    }

    fn complie(&mut self, program_start: Instant) {
        let now = program_start.elapsed();
        println!("{:?} {} has taken a dongle", now, self.coder_number);

        self.last_compile = now;
        println!("{:?} {} is compiling", now, self.coder_number);
        sleep(self.time_to_compile);
        self.compiles_left -= 1;
    }

    fn debug(&mut self, program_start: Instant) {
        let now = program_start.elapsed();
        println!("{:?} {} is debugging", now, self.coder_number);
        sleep(self.time_to_debug);
    }

    fn refactor(&mut self, program_start: Instant) {
        let now = program_start.elapsed();
        println!("{:?} {} is refactoring", now, self.coder_number);
        sleep(self.time_to_debug);
    }
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

fn main() {
    let program_args = parse_args();

    let mut thread_handles = vec![];
    let program_start = Instant::now();

    let mut dongles = Vec::new();
    for _ in 0..program_args.number_of_coders {
        dongles.push(Arc::new((
            Mutex::new(program_start.elapsed()),
            Condvar::new(),
        )));
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
                coder.complie(program_start);
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
