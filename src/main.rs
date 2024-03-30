mod cmd_line;
mod siphon;

use cmd_line::get_args;

fn main() {
    let args = get_args();
    let unit_pack = siphon::get_unit_iter(&args.cnf).unwrap();
    for unit in unit_pack.unit_iter {
        println!("{unit}");
    }
}
