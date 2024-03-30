use nix::sys::stat;
use nix::unistd::mkfifo;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use tempfile::tempdir;

fn parse_unit(line: &str) -> Option<i64> {
    let split: Vec<_> = line.trim().split(' ').collect();
    if split.len() != 2 {
        return None;
    }
    match split[0].parse() {
        Ok(i) => Some(i),
        Err(_) => panic!("Falsely found a unit with input line in the proof {line}"),
    }
}

pub fn get_units() {
    let tmp_dir = tempdir().unwrap();
    let fifo_path = tmp_dir.path().join("cadical.pipe");
    mkfifo(&fifo_path, stat::Mode::S_IRWXU).unwrap();

    println!("here");
    Command::new("cadical")
        .args([
            "--binary=false",
            "formulas/cross-10-59-UNS.cnf",
            &fifo_path.to_str().unwrap(),
        ])
        .stdout(Stdio::null())
        .spawn()
        .unwrap();

    let f = File::open(fifo_path).unwrap();
    let reader = BufReader::new(f);
    for line in reader.lines() {
        if let Some(i) = parse_unit(&line.unwrap()) {
            println!("Found unit {i}")
        }
    }
}
