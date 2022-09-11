mod csp;

#[cfg(test)]
mod tests;

use crate::{Coordinate, MSMatrix, MineSweeper, Result};
use std::cell::Ref;

pub trait Solver<M>
where
    M: MineSweeper,
{
    fn new() -> Self;
    fn solve(board: M, start_from: Coordinate) -> Result<bool>;
}

pub struct NonDeterministic {}

impl<M> Solver<M> for NonDeterministic
where
    M: MineSweeper,
{
    fn new() -> Self {
        Self {}
    }

    fn solve(_: M, _: Coordinate) -> Result<bool> {
        Ok(true)
    }
}
