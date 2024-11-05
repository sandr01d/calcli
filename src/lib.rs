#![warn(clippy::pedantic)]

use clap::Parser;
use std::{
    io::{self, Write},
    time::Instant,
};

mod exercise;
use exercise::Excercise;

#[derive(Parser, Debug, Clone, Copy)]
#[command(version, about, long_about=None)]
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

    println!("\n\x1B[1m === Results ===\x1B[22m\n");
    let mut correct = 0;
    for ex in exercises {
        let result = ex.result();
        // TODO pretty print
        if result == ex.answer {
            correct += 1;
            println!("{} {} ✅", ex.question(), result);
        } else {
            println!("{} {} ❌ ({})", ex.question(), result, ex.answer);
        }
    }
    println!(
        "\n{correct}/{} correct, in {}.{}s",
        args.count,
        duration.as_secs(),
        duration.subsec_millis()
    );
    Ok(())
}

/// Prints out the question for the `exercise` and reads an answer to it from stdin.
fn ask(exercise: &mut Excercise) -> io::Result<()> {
    let mut buf = String::new();
    print!("{} ", exercise.question());
    io::stdout().flush()?;
    io::stdin().read_line(&mut buf)?;
    // parse() can return an error, e.g. when letters where entered
    match buf.trim().parse() {
        Ok(i) => exercise.answer = i,
        Err(e) => {
            eprintln!("Input invalid: {e}");
            return ask(exercise);
        }
    }
    Ok(())
}
