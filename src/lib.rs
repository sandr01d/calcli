use clap::Parser;

#[derive(Parser)]
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
