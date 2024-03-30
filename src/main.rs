mod siphon;
mod cmd_line;

use std::io::BufRead;

use clap::Parser;
use cmd_line::Args;

fn main() {
    let args = Args::parse();
    let unit_reader = siphon::get_drat_reader(&args.cnf).unwrap();

    for unit in unit_reader.lines() {
        if let Some(i) = siphon::parse_unit(&unit.unwrap()) {
            println!("{i}");
        }
    }
}
