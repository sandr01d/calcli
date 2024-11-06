use std::fmt::Display;
use rand::{distributions::Standard, prelude::Distribution, Rng};

#[derive(PartialEq)]
pub enum Operation {
    Plus,
    Minus,
    Mul,
    Div,
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sign = match &self {
            Operation::Plus => '+',
            Operation::Minus => '-',
            Operation::Mul => '×',
            Operation::Div => '/',
        };
        write!(f, "{sign}")
    }
}

impl Distribution<Operation> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Operation {
        match rng.gen_range(0..=3) {
            0 => Operation::Plus,
            1 => Operation::Minus,
            2 => Operation::Mul,
            3 => Operation::Div,
            _ => panic!("Invalid random number for operator!"),
        }
    }
}
