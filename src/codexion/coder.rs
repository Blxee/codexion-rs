use crate::args::Args;

pub struct Coder {
    id: u32,
}

impl Coder {
    pub fn new(id: u32, args: &Args) -> Self {
        Self { id }
    }
}
