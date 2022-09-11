mod csp;

#[cfg(test)]
mod tests;

use crate::{Coordinate, MineSweeper, Result};
use std::cell::RefCell;

pub trait Solver<M: MineSweeper> {
    // fn new(board: &'a mut T) -> Self;
    fn solve(board: RefCell<M>, start_from: Coordinate) -> Result<bool>;
}
