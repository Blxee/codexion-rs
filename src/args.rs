use std::{env::Args as ProgramArgs, num::ParseIntError};

#[derive(Debug)]
pub struct Args {
    pub number_of_coders: u32,
    pub time_to_burnout: u32,
    pub time_to_compile: u32,
    pub time_to_debug: u32,
    pub time_to_refactor: u32,
    pub number_of_compiles_required: u32,
    pub dongle_cooldown: u32,
    pub scheduler: Scheduler,
}

type Scheduler = String;

#[derive(Debug)]
pub enum ArgsError {
    InvalidArgumentCount,
    InvalidNumber {
        argument: &'static str,
        source: ParseIntError,
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

        let time_to_burnout: u32 = args[2].parse().map_err(|source| ArgsError::InvalidNumber {
            argument: "time_to_burnout",
            source,
        })?;

        let time_to_compile: u32 = args[3].parse().map_err(|source| ArgsError::InvalidNumber {
            argument: "time_to_compile",
            source,
        })?;

        let time_to_debug: u32 = args[4].parse().map_err(|source| ArgsError::InvalidNumber {
            argument: "time_to_debug",
            source,
        })?;

        let time_to_refactor: u32 = args[5].parse().map_err(|source| ArgsError::InvalidNumber {
            argument: "time_to_refactor",
            source,
        })?;

        let number_of_compiles_required: u32 =
            args[6].parse().map_err(|source| ArgsError::InvalidNumber {
                argument: "number_of_compiles_required",
                source,
            })?;

        let dongle_cooldown: u32 = args[7].parse().map_err(|source| ArgsError::InvalidNumber {
            argument: "dongle_cooldown",
            source,
        })?;

        let scheduler: String = match args[8].as_str() {
            "fifo" => "fifo".to_string(),
            "edf" => "edf".to_string(),
            _ => return Err(ArgsError::InvalidScheduler),
        };

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
