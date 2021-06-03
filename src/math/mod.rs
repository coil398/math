pub mod formula;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MathError {
    #[error("Invalid parameters: {0}")]
    InvalidParameters(String),
}

pub fn sum(n: u64) -> u64 {
    let mut res = 0;
    for k in 1..=n {
        res += k;
    }
    res
}

pub fn triangle(n: u64) -> u64 {
    let mut res = 0;
    for k in 0..n {
        res += k + 1;
    }
    res
}
