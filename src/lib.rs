#![warn(clippy::pedantic)]

use clap::Parser;
use std::{
    io::{self, Write},
    time::{Duration, Instant},
};
use tabled::{
    builder::Builder,
    settings::{Alignment, Style, Width},
};

mod exercise;
use exercise::Excercise;

#[derive(Parser, Debug, Clone, Copy)]
#[command(version, about)]
pub struct Cli {
    /// Number of excercises
    #[arg(short, long, default_value_t = 10)]
    pub count: usize,
    /// Smallest allowed number in equasions
    #[arg(long, default_value_t = 0)]
    pub min: u8,
    /// Largest allowed number in equasions
    #[arg(long, default_value_t = 100, value_parser = clap::value_parser!(u8).range(1..))]
    pub max: u8,
}

/// Main entry point
///
/// # Errors
///
/// Returns an error when either `Stdout::flush()` or `Stdin::read_line()` fail
/// which would indicate an error on the underlying OS.
pub fn run(args: &Cli) -> io::Result<()> {
    let mut rng = rand::thread_rng();
    let mut exercises = Vec::with_capacity(args.count);
    let now = Instant::now();
    for _ in 0..args.count {
        // Generate random exercise
        let mut ex = Excercise::new(&mut rng, args.min, args.max);
        ask(&mut ex)?;
        exercises.push(ex);
    }
    let duration = now.elapsed();
    print_results(&exercises, duration);
    Ok(())
}

/// Prints the results of the run in a table
fn print_results(exercises: &Vec<Excercise>, duration: Duration) {
    // number of correct responses
    let mut correct = 0;

    // builder for the header
    let mut builder = Builder::default();
    // ansi escope codes are for bold text
    builder.push_record(["\x1B[1mResults\x1B[22m"]);
    let mut header = builder.build();

    // builder for the table
    let mut builder = Builder::default();
    builder.push_record(["Exercise", "Solution", "Outcome", "Response"]);

    // fill the table
    for ex in exercises {
        let solution = ex.solve();
        let icon = if solution == ex.response {
            correct += 1;
            String::from("✅")
        } else {
            String::from("❌")
        };
        builder.push_record([
            ex.question(),
            solution.to_string(),
            icon,
            ex.response.to_string(),
        ]);
    }

    let mut table = builder.build();
    table.with(Style::rounded());

    // styling header after the table because we want to make it the same width
    header
        .with(Style::extended())
        .with(Width::increase(table.total_width()))
        .with(Alignment::center());

    println!();
    println!("{header}");
    println!("{table}");
    println!(
        "\n{correct}/{} correct, in {}.{}s",
        exercises.len(),
        duration.as_secs(),
        duration.subsec_millis()
    );
}

/// Prints out the question for the `exercise` and reads an answer to it from stdin.
fn ask(exercise: &mut Excercise) -> io::Result<()> {
    let mut buf = String::new();
    print!("{}  ", exercise.question());
    io::stdout().flush()?;
    io::stdin().read_line(&mut buf)?;
    // parse() can return an error, e.g. when letters where entered
    match buf.trim().parse() {
        Ok(i) => exercise.response = i,
        Err(e) => {
            eprintln!("Input invalid: {e}");
            return ask(exercise);
        }
    }
    Ok(())
}
