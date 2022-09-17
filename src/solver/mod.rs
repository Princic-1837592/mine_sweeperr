pub use csp::CSPSolver;
pub use single_point::SPSolver;

use crate::{Coordinate, MineSweeper, Result};

mod csp;
mod single_point;

#[cfg(test)]
mod tests;


pub trait Solver<M>
where
    M: MineSweeper,
{
    fn solve(ms: M, start_from: Coordinate) -> Result<bool>;
}

pub struct NonDeterministic {}

impl<M> Solver<M> for NonDeterministic
where
    M: MineSweeper,
{
    fn solve(_: M, _: Coordinate) -> Result<bool> {
        Ok(true)
    }
}
