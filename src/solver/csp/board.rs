use std::cell::RefCell;

use crate::{utils::iter_neighbors, Cell, CellState, Coordinate, MineSweeper};

use super::Constraint;

pub(crate) const UNKNOWN: i8 = -5;
pub(crate) const CONSTRAINED: i8 = -4;
pub(crate) const MARKED: i8 = -3;
pub(crate) const CLEAR: i8 = 0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct BoardCell {
    pub cell: Cell,
    pub coordinate: Coordinate,
    pub state: i8,
    pub boundary_level: u8,
    pub test_assignment: i8,
}

impl BoardCell {
    fn new(cell: Cell, coordinate: Coordinate) -> Self {
        BoardCell {
            cell,
            coordinate,
            state: UNKNOWN,
            boundary_level: 0,
            test_assignment: -1,
        }
    }
}

pub(crate) struct Board<'a> {
    unknown: usize,
    // maybe useless
    constrained: usize,
    // maybe useless
    mine: usize,
    // maybe useless
    clear: usize,
    cells: Vec<Vec<RefCell<BoardCell>>>,
    phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> Board<'a> {
    pub fn new(ms: impl MineSweeper) -> Self {
        let mut cells = vec![Vec::with_capacity(ms.width()); ms.height()];
        for (r, row) in cells.iter_mut().enumerate() {
            for c in 0..ms.width() {
                row.push(RefCell::new(BoardCell::new(
                    ms.get_cell((r, c)).unwrap(),
                    (r, c),
                )));
            }
        }
        Board {
            unknown: ms.width() * ms.height(),
            constrained: 0,
            mine: 0,
            clear: 0,
            cells,
            phantom: std::marker::PhantomData,
        }
    }

    pub fn enumerate_boundary(&mut self, level: u8) -> Vec<&RefCell<BoardCell>> {
        let mut result = Vec::with_capacity(self.unknown as usize);
        for row in &self.cells {
            for cell in row {
                if cell.borrow().state == UNKNOWN && cell.borrow().boundary_level == level {
                    result.push(cell);
                }
            }
        }
        result
    }

    pub fn enumerate_max_boundary(&mut self) -> Vec<&RefCell<BoardCell>> {
        let mut max = 0;
        for row in &self.cells {
            for cell in row {
                if cell.borrow().state == UNKNOWN && cell.borrow().boundary_level > max {
                    max = cell.borrow().boundary_level;
                }
            }
        }
        if max == 0 {
            Vec::new()
        } else {
            self.enumerate_boundary(max)
        }
    }

    fn enumerate_min_boundary(&mut self) -> Vec<&RefCell<BoardCell>> {
        let mut min = u8::MAX;
        for row in &self.cells {
            for cell in row {
                if cell.borrow().state == UNKNOWN
                    && cell.borrow().boundary_level > 0
                    && cell.borrow().boundary_level < min
                {
                    min = cell.borrow().boundary_level;
                }
            }
        }
        if min == 0 {
            Vec::new()
        } else {
            self.enumerate_boundary(min)
        }
    }

    fn enumerate_unknown(&mut self) -> Vec<&RefCell<BoardCell>> {
        let mut result = Vec::with_capacity(self.unknown);
        for row in &self.cells {
            for cell in row {
                if cell.borrow().state == UNKNOWN {
                    result.push(cell);
                }
            }
        }
        result
    }

    fn enumerate_edges(&mut self) -> Vec<&RefCell<BoardCell>> {
        let mut result = Vec::new();
        for r in 1..(self.cells.len() - 1) {
            if self.cells[r][0].borrow().state < CONSTRAINED {
                result.push(&self.cells[r][0]);
            }
            if self.cells[r][self.cells[r].len() - 1].borrow().state < CONSTRAINED {
                result.push(&self.cells[r][self.cells[r].len() - 1]);
            }
        }
        for c in 1..(self.cells[0].len() - 1) {
            if self.cells[0][c].borrow().state < CONSTRAINED {
                result.push(&self.cells[0][c]);
            }
            if self.cells[self.cells.len() - 1][c].borrow().state < CONSTRAINED {
                result.push(&self.cells[self.cells.len() - 1][c]);
            }
        }
        result
    }

    pub fn enumerate_corners(&mut self) -> Vec<&RefCell<BoardCell>> {
        let mut result = Vec::with_capacity(4);
        if self.cells[0][0].borrow().state < CONSTRAINED {
            result.push(&self.cells[0][0]);
        }
        if self.cells[0][self.cells[0].len() - 1].borrow().state < CONSTRAINED {
            result.push(&self.cells[0][self.cells[0].len() - 1]);
        }
        if self.cells[self.cells.len() - 1][0].borrow().state < CONSTRAINED {
            result.push(&self.cells[self.cells.len() - 1][0]);
        }
        if self.cells[self.cells.len() - 1][self.cells[0].len() - 1]
            .borrow()
            .state
            < CONSTRAINED
        {
            result.push(&self.cells[self.cells.len() - 1][self.cells[0].len() - 1]);
        }
        result
    }

    pub fn new_constraint(&'a self, coord @ (r, c): Coordinate) -> Option<Constraint<'a>> {
        if self.cells[r][c].borrow().state < 0 {
            return None;
        }
        let mut state = self.cells[r][c].borrow().state;
        let mut constraint = Constraint::new();
        let mut constant = state as u8;
        for (r, c) in iter_neighbors(coord, self.cells.len(), self.cells[0].len()).unwrap() {
            state = self.cells[r][c].borrow().state;
            if state < 0 {
                if state == MARKED {
                    constant -= 1;
                } else {
                    constraint.add_variable(&self.cells[r][c]);
                    self.cells[r][c].borrow_mut().state = CONSTRAINED;
                }
            }
        }
        constraint.set_constant(constant);
        Some(constraint)
    }

    // in teoria non dovrebbe mai provare ad aprire una cella flaggata o già aperta (?)
    pub fn open(&mut self, coord @ (r, c): Coordinate) {
        self.cells[r][c].borrow_mut().cell.state = CellState::Open;
    }

    pub fn flag(&mut self, coord @ (r, c): Coordinate) {
        self.cells[r][c].borrow_mut().cell.state = CellState::Flagged;
    }
}
