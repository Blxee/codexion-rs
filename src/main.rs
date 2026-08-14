mod args;
mod codexion;
mod logging;
use crate::{args::Args, codexion::Codexion};
use std::env::args;

fn main() {
    let args: Args = match args().try_into() {
        Ok(args) => args,
        Err(err) => {
            eprintln!("{err}");
            return print_usage();
        }
    };

    dbg!(&args);

    let codexion = Codexion::new(args);
    codexion.start();
}

fn print_usage() {
    const FG_GREEN: &'static str = "\x1b[32m";
    const FG_BLUE: &'static str = "\x1b[34m";
    const BOLD: &'static str = "\x1b[1m";
    const DIM: &'static str = "\x1b[2m";
    const RESET: &'static str = "\x1b[0m";

    eprintln!("\
Usage:
    {FG_BLUE}{BOLD}codexion-rs{RESET}  {BOLD}number_of_coders time_to_burnout time_to_compile time_to_debug time_to_refactor number_of_compiles_required dongle_cooldown scheduler{RESET}

    {FG_GREEN}number_of_coders{RESET} ({DIM}u32{RESET}): number of coders and threads.

    {FG_GREEN}time_to_burnout{RESET} ({DIM}u64{RESET}): millis until a coder burns out, if he doesn't compile.

    {FG_GREEN}time_to_compile{RESET} ({DIM}u64{RESET}): millis it takes to compile.

    {FG_GREEN}time_to_debug{RESET} ({DIM}u64{RESET}): millis it takes to debug.

    {FG_GREEN}time_to_refactor{RESET} ({DIM}u64{RESET}): millis it takes to refactor.

    {FG_GREEN}number_of_compiles_required{RESET} ({DIM}u32{RESET}): number of successful compiles required for a coder to stop.

    {FG_GREEN}dongle_cooldown{RESET} ({DIM}u64{RESET}): millis it takes for a dongle to cooldown after being used.

    {FG_GREEN}scheduler{RESET} ({DIM}fifo/edf{RESET}): scheduling strategy, FIFO (First In First Out), EDF (Earliest Deadline First)\
");
}
