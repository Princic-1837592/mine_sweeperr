use super::constraint::Constraint;
use crate::solver::csp::board::Board;
use crate::{solver::Solver, CellContent, Coordinate, MSMatrix, MineSweeper, Result};
use std::cell::{Ref, RefCell};

pub(crate) struct CSPSolver<M: MineSweeper> {
    constraints: Vec<Constraint>,
    ms: M,
    board: Board,
}

impl<M: MineSweeper> CSPSolver<M> {
    fn new(ms: M) -> Self {
        let board = Board::new(ms.height(), ms.width());
        CSPSolver {
            //todo consider using with_capacity
            constraints: Vec::new(),
            ms,
            board,
        }
    }

    fn solve(&mut self, start_from: Coordinate) -> Result<bool> {
        if let CellContent::Mine = self.ms.get_cell(start_from)?.content {
            return Ok(false);
        }
        Ok(false)
        // unimplemented!()
    }
}

impl<M: MineSweeper> Solver<M> for CSPSolver<M> {
    fn new() -> Self {
        todo!()
    }

    fn solve(board: M, start_from: Coordinate) -> Result<bool> {
        Self::new(board).solve(start_from)
    }
}
