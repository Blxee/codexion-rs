mod args;
mod coder;
mod dongle;
mod quantum_table;

use crate::args::parse_args;
use crate::quantum_table::QuantumTable;

fn main() {
    let program_args = parse_args();

    let quantum_table = QuantumTable::new(program_args);

    quantum_table.start_coding();
}
