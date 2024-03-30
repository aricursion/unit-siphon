use nix::sys::stat;
use nix::unistd::mkfifo;
use std::fs::File;
use std::io::BufReader;
use std::io::{self, BufRead};
use std::process::{Child, Command, Stdio};
use tempfile::tempdir;

type Unit = i64;

pub struct UnitPackage {
    pub unit_iter: Box<dyn Iterator<Item = Unit>>,
    pub proc: Child,
}

impl UnitPackage {
    fn new(child: Child, iter: impl Iterator<Item = Unit> + 'static) -> Self {
        UnitPackage {
            unit_iter: Box::new(iter),
            proc: child,
        }
    }
}
fn parse_unit(line: &str) -> Option<Unit> {
    let split: Vec<_> = line.trim().split(' ').collect();
    if split.len() != 2 {
        return None;
    }
    match split[0].parse() {
        Ok(i) => Some(i),
        Err(_) => panic!("Falsely found a \"unit\" with input line in the proof {line}"),
    }
}

pub fn get_unit_iter(cnf_file_loc: &str) -> io::Result<UnitPackage> {
    let tmp_dir = tempdir()?;
    let fifo_path = tmp_dir.path().join("cadical.pipe");
    mkfifo(&fifo_path, stat::Mode::S_IRWXU)?;

    let child = Command::new("cadical")
        .args(["--binary=false", cnf_file_loc, &fifo_path.to_str().unwrap()])
        .stdout(Stdio::null())
        .spawn()?;

    let f = File::open(fifo_path)?;
    let buf = BufReader::new(f);
    let lines = buf.lines();
    let out = UnitPackage::new(child, lines.filter_map(|line| parse_unit(&line.unwrap())));
    Ok(out)
}
