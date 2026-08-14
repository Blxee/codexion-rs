use std::{
    env::Args as ProgramArgs, error::Error, fmt::Display, num::ParseIntError, time::Duration,
};

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
        min_value: u64,
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

        let number_of_coders: u32 = Self::parse_u32(&args[1], "number_of_coders")?;
        if number_of_coders < 1 {
            return Err(ArgsError::InvalidNumberRange {
                argument: "number_of_coders",
                min_value: 1,
            });
        }

        let time_to_burnout: u64 = Self::parse_u64(&args[2], "time_to_burnout")?;
        let time_to_compile: u64 = Self::parse_u64(&args[3], "time_to_compile")?;
        let time_to_debug: u64 = Self::parse_u64(&args[4], "time_to_debug")?;
        let time_to_refactor: u64 = Self::parse_u64(&args[5], "time_to_refactor")?;

        let number_of_compiles_required: u32 =
            Self::parse_u32(&args[6], " number_of_compiles_required")?;
        if number_of_compiles_required < 1 {
            return Err(ArgsError::InvalidNumberRange {
                argument: "number_of_compiles_required",
                min_value: 1,
            });
        }

        let dongle_cooldown: u64 = Self::parse_u64(&args[7], "dongle_cooldown")?;

        let scheduler = args[8].as_str().try_into()?;

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

impl Args {
    fn parse_u64(value: &String, arg_name: &'static str) -> Result<u64, ArgsError> {
        value.parse().map_err(|source| ArgsError::InvalidNumber {
            argument: arg_name,
            source,
        })
    }

    fn parse_u32(value: &String, arg_name: &'static str) -> Result<u32, ArgsError> {
        value.parse().map_err(|source| ArgsError::InvalidNumber {
            argument: arg_name,
            source,
        })
    }
}

impl TryFrom<&str> for Scheduler {
    type Error = ArgsError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "fifo" => Ok(Scheduler::FIFO),
            "edf" => Ok(Scheduler::EDF),
            _ => Err(ArgsError::InvalidScheduler),
        }
    }
}

impl Display for ArgsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgsError::InvalidArgumentCount => write!(f, "Error: invalid argument count"),
            ArgsError::InvalidNumber { argument, source } => write!(
                f,
                "Error: invalid number for argument '{argument}': {source}"
            ),
            ArgsError::InvalidNumberRange {
                argument,
                min_value,
            } => write!(
                f,
                "Error: invalid number range for argument '{argument}' (min_value: {min_value})"
            ),
            ArgsError::InvalidScheduler => write!(f, "Error: invalid scheduler"),
        }
    }
}

impl Error for ArgsError {}
