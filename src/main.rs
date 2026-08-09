mod args;
use crate::args::Args;
use std::env::args;

fn main() {
    let args: Args = args().try_into().unwrap();

    dbg!(args);
}
