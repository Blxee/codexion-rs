mod args;
use crate::args::Args;
use std::env::args as program_args;

fn main() {
    let args: Args = program_args().try_into().unwrap();

    dbg!(args);
}
