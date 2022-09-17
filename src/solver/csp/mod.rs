use board::Board;
use constraint::Constraint;

use crate::{solver::Solver, CellContent, Coordinate, MineSweeper, Result};

mod board;
mod constraint;
#[cfg(test)]
mod tests;

pub struct CSPSolver<'a> {
    constraints: Vec<Constraint<'a>>,
    board: Board<'a>,
}

impl<'a> CSPSolver<'a> {
    fn new(ms: impl MineSweeper) -> Self {
        let board = Board::new(ms);
        CSPSolver {
            //todo consider using with_capacity
            constraints: Vec::new(),
            board,
        }
    }

    fn solve(&mut self, start_from: Coordinate) -> Result<bool> {
        Ok(false)
        // unimplemented!()
    }
}

impl<'a, M: MineSweeper> Solver<M> for CSPSolver<'a> {
    fn solve(ms: M, start_from: Coordinate) -> Result<bool> {
        if let CellContent::Mine = ms.get_cell(start_from)?.content {
            return Ok(false);
        }
        Self::new(ms).solve(start_from)
    }
}
