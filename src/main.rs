use calcli::Cli;
use clap::Parser;
use std::process;

fn main() {
    let cli = Cli::parse();
    // Check if the input the arguments are valid
    if cli.min > cli.max {
        eprintln!("Min can not be bigger than max");
        process::exit(1);
    }
    if let Err(e) = calcli::run(&cli) {
        eprintln!("Failed: {e}");
        process::exit(1);
    }
}
