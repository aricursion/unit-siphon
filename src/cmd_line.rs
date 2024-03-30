use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    #[arg(short, long)]
    pub cnf: String,
}

pub fn get_args() -> Args {
    Args::parse()
}
