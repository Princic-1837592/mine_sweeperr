mod csp;
mod single_point;

#[cfg(test)]
mod tests;

use crate::{Coordinate, MineSweeper, Result};
pub use csp::CSPSolver;

pub trait Solver<M>
where
    M: MineSweeper,
{
    fn new() -> Self;
    fn solve(ms: M, start_from: Coordinate) -> Result<bool>;
}

pub struct NonDeterministic {}

impl<M> Solver<M> for NonDeterministic
where
    M: MineSweeper,
{
    fn new() -> Self {
        NonDeterministic {}
    }

    fn solve(_: M, _: Coordinate) -> Result<bool> {
        Ok(true)
    }
}
