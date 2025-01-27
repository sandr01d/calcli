use rand::Rng;

mod operation;
use operation::Operation;

pub struct Excercise {
    /// Left hand side of the equasion
    lhs: u8,
    /// Right hand side of the equasion
    rhs: u8,
    /// Operation
    op: Operation,
    /// Answer provided by the user
    pub response: i32,
}

impl Excercise {
    /// Generates a new random exercise with the given limits for `lhs` and `rhs`
    pub fn new<R: Rng>(rng: &mut R, min: u8, max: u8) -> Excercise {
        let op = rng.random();
        let lhs = rng.random_range(min..=max);
        let rhs = if op == Operation::Div {
            gen_divisor(rng, min, max, lhs)
        } else {
            rng.random_range(min..=max)
        };
        Excercise {
            lhs,
            rhs,
            op,
            response: 0,
        }
    }

    /// The question, as displayed to the user
    pub fn question(&self) -> String {
        format!("{:2}  {}  {:2}  =", &self.lhs, &self.op, &self.rhs)
    }

    /// The correct result of the exercise
    pub fn solve(&self) -> i32 {
        let lhs = i32::from(self.lhs);
        let rhs = i32::from(self.rhs);
        match &self.op {
            Operation::Plus => lhs + rhs,
            Operation::Minus => lhs - rhs,
            Operation::Mul => lhs * rhs,
            Operation::Div => lhs / rhs,
        }
    }
}

/// Generates a random divisor for the given dividend.
///
/// Returns a random number between `min` and `dividend` when `dividend` is
/// non-zero. Returns a random number between `min` and `max` when `dividend`
/// is zero.
///
/// * `rng` random number generator
/// * `min` minimum allowed value for the result
/// * `max` maximum allowed value for the result (only relevant when
///   `dividend == 0`)
/// * `dividend` the number to generate a divisor for
fn gen_divisor<R: Rng>(rng: &mut R, min: u8, max: u8, dividend: u8) -> u8 {
    // Min must be at least 1, to avoid divisor becoming 0
    let min = min.max(1);
    // When the dividend is 0, the loop below would have an empty range, due to
    // min being bigger than dividend and thread would panic. In this special
    // case we allow the divisor to be bigger than the dividend.
    if dividend == 0 {
        return rng.random_range(min..=max);
    }
    loop {
        let divisor = rng.random_range(min..=dividend);
        if dividend % divisor == 0 {
            return divisor;
        }
    }
}
