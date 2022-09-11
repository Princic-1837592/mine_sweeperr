use super::constraint::Constraint;
use crate::solver::csp::board::Board;
use crate::{solver::Solver, CellContent, Coordinate, MineSweeper, Result};
use std::cell::RefCell;

pub(crate) struct CSPSolver<M: MineSweeper> {
    constraints: Vec<Constraint>,
    ms: RefCell<M>,
    board: Board,
}

impl<'a, M: MineSweeper> CSPSolver<M> {
    fn new(ms: RefCell<M>) -> Self {
        let board = Board::new(ms.borrow().height(), ms.borrow().width());
        CSPSolver {
            //todo consider using with_capacity
            constraints: Vec::new(),
            ms,
            board,
        }
    }

    fn solve(&mut self, start_from: Coordinate) -> Result<bool> {
        if let CellContent::Mine = self.ms.borrow().get_cell(start_from)?.content {
            return Ok(false);
        }
        Ok(true)
        // unimplemented!()
    }
}

impl<'a, M: MineSweeper> Solver<M> for CSPSolver<M> {
    // fn new() -> Self {
    //     CSPSolver {
    //         //todo consider using with_capacity
    //         constraints: Vec::new(),
    //         board,
    //     }
    // }

    fn solve(board: RefCell<M>, start_from: Coordinate) -> Result<bool> {
        Self::new(board).solve(start_from)
    }
}
