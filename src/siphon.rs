use nix::sys::stat;
use nix::unistd::mkfifo;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::process::{Command, Stdio};
use tempfile::tempdir;

pub fn parse_unit(line: &str) -> Option<i64> {
    let split: Vec<_> = line.trim().split(' ').collect();
    if split.len() != 2 {
        return None;
    }
    match split[0].parse() {
        Ok(i) => Some(i),
        Err(_) => panic!("Falsely found a \"unit\" with input line in the proof {line}"),
    }
}

pub fn get_drat_reader(cnf_file_loc: &str) -> io::Result<BufReader<File>> {
    let tmp_dir = tempdir()?;
    let fifo_path = tmp_dir.path().join("cadical.pipe");
    mkfifo(&fifo_path, stat::Mode::S_IRWXU)?;

    Command::new("cadical")
        .args(["--binary=false", cnf_file_loc, &fifo_path.to_str().unwrap()])
        .stdout(Stdio::null())
        .spawn()?;

    let f = File::open(fifo_path)?;
    Ok(BufReader::new(f))
}
