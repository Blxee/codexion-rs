mod args;
mod codexion;
mod logging;
use crate::{args::Args, codexion::Codexion};
use std::env::args;

fn main() {
    let args: Args = match args().try_into() {
        Ok(args) => args,
        Err(err) => {
            return eprintln!("{err:?}");
        }
    };

    dbg!(&args);

    let codexion = Codexion::new(args);
    codexion.start();
}
