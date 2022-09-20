#![allow(unused)]

pub use csp::CSPSolver;
pub use single_point::SPSolver;

use crate::{Coordinate, MineSweeper, Result};

mod csp;
mod single_point;

#[cfg(test)]
mod tests;

pub trait Solver {
    fn solve<M: MineSweeper + Clone>(ms: &M, start_from: Coordinate) -> Result<bool>;
    fn guessed() -> usize {
        0
    }
}

pub struct NonDeterministic {}

impl Solver for NonDeterministic {
    fn solve<M: MineSweeper>(_: &M, _: Coordinate) -> Result<bool> {
        Ok(true)
    }
}
