use std::env::Args as ProgramArgs;

#[derive(Debug)]
pub struct Args {
    number_of_coders: u32,
    time_to_burnout: u32,
    time_to_compile: u32,
    time_to_debug: u32,
    time_to_refactor: u32,
    number_of_compiles_required: u32,
    dongle_cooldown: u32,
    scheduler: Scheduler,
}

type Scheduler = String;

impl TryFrom<ProgramArgs> for Args {
    type Error = &'static str;

    fn try_from(args: ProgramArgs) -> Result<Self, Self::Error> {
        let args: Vec<_> = args.collect();

        if args.len() != 9 {
            return Err("Invalid amount of arguments");
        }

        let number_of_coders = args[1]
            .parse()
            .map_err(|_| "number_of_coders is not a valid number")?;

        let time_to_burnout: u32 = args[2]
            .parse()
            .map_err(|_| "time_to_burnoutis not a valid number")?;

        let time_to_compile: u32 = args[3]
            .parse()
            .map_err(|_| "time_to_compileis not a valid number")?;

        let time_to_debug: u32 = args[4]
            .parse()
            .map_err(|_| "time_to_debugis not a valid number")?;

        let time_to_refactor: u32 = args[5]
            .parse()
            .map_err(|_| "time_to_refactoris not a valid number")?;

        let number_of_compiles_required: u32 = args[6]
            .parse()
            .map_err(|_| "number_of_compiles_requiredis not a valid number")?;

        let dongle_cooldown: u32 = args[7]
            .parse()
            .map_err(|_| "dongle_cooldownis not a valid number")?;

        let scheduler: String = match args[8].as_str() {
            "fifo" => "fifo".to_string(),
            "edf" => "edf".to_string(),
            _ => return Err("Invalid scheduler"),
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
