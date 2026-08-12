use std::{env::Args as ProgramArgs, num::ParseIntError, time::Duration};

#[derive(Clone, Copy, Debug)]
pub struct Args {
    pub number_of_coders: u32,
    pub time_to_burnout: Duration,
    pub time_to_compile: Duration,
    pub time_to_debug: Duration,
    pub time_to_refactor: Duration,
    pub number_of_compiles_required: u32,
    pub dongle_cooldown: Duration,
    pub scheduler: Scheduler,
}

#[derive(Debug, Clone, Copy)]
pub enum Scheduler {
    FIFO,
    EDF,
}

#[derive(Debug)]
pub enum ArgsError {
    InvalidArgumentCount,
    InvalidNumber {
        argument: &'static str,
        source: ParseIntError,
    },
    InvalidNumberRange {
        argument: &'static str,
        min_value: u32,
    },
    InvalidScheduler,
}

impl TryFrom<ProgramArgs> for Args {
    type Error = ArgsError;

    fn try_from(args: ProgramArgs) -> Result<Self, Self::Error> {
        let args: Vec<_> = args.collect();

        if args.len() != 9 {
            return Err(ArgsError::InvalidArgumentCount);
        }

        let number_of_coders = args[1].parse().map_err(|source| ArgsError::InvalidNumber {
            argument: "number_of_coders",
            source,
        })?;

        if number_of_coders < 1 {
            return Err(ArgsError::InvalidNumberRange {
                argument: "number_of_coders",
                min_value: 1,
            });
        }

        let time_to_burnout: u64 = args[2].parse().map_err(|source| ArgsError::InvalidNumber {
            argument: "time_to_burnout",
            source,
        })?;

        let time_to_compile: u64 = args[3].parse().map_err(|source| ArgsError::InvalidNumber {
            argument: "time_to_compile",
            source,
        })?;

        let time_to_debug: u64 = args[4].parse().map_err(|source| ArgsError::InvalidNumber {
            argument: "time_to_debug",
            source,
        })?;

        let time_to_refactor: u64 = args[5].parse().map_err(|source| ArgsError::InvalidNumber {
            argument: "time_to_refactor",
            source,
        })?;

        let number_of_compiles_required: u32 =
            args[6].parse().map_err(|source| ArgsError::InvalidNumber {
                argument: "number_of_compiles_required",
                source,
            })?;

        let dongle_cooldown: u64 = args[7].parse().map_err(|source| ArgsError::InvalidNumber {
            argument: "dongle_cooldown",
            source,
        })?;

        let scheduler = match args[8].as_str() {
            "fifo" => Scheduler::FIFO,
            "edf" => Scheduler::EDF,
            _ => return Err(ArgsError::InvalidScheduler),
        };

        let time_to_burnout = Duration::from_millis(time_to_burnout);
        let time_to_compile = Duration::from_millis(time_to_compile);
        let time_to_debug = Duration::from_millis(time_to_debug);
        let time_to_refactor = Duration::from_millis(time_to_refactor);
        let dongle_cooldown = Duration::from_millis(dongle_cooldown);

        Ok(Self {
            number_of_coders,
            time_to_burnout,
            time_to_compile,
            time_to_debug,
            time_to_refactor,
            number_of_compiles_required,
            dongle_cooldown,
            scheduler,
        })
    }
}
