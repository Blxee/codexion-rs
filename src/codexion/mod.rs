mod coder;
use std::thread;

use crate::args::Args;
use crate::codexion::coder::Coder;

pub struct Codexion {
    coders: Vec<Coder>,
}

impl Codexion {
    pub fn new(args: &Args) -> Self {
        let coders = (0..args.number_of_coders)
            .map(|id| Coder::new(id, args))
            .collect();
        Self { coders }
    }

    pub fn start(self) {}
}
