mod args;
use crate::args::Args;
use std::env::args;

fn main() {
    let args: Args = match args().try_into() {
        Ok(args) => args,
        Err(err) => {
            return eprintln!("{err:?}");
        }
    };

    dbg!(args);
}
