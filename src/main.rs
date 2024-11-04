#![warn(clippy::pedantic)]

use calcli::Cli;
use clap::Parser;

fn main() {
    let cli = Cli::parse();
    println!("I will ask {} questions", cli.count);
}
