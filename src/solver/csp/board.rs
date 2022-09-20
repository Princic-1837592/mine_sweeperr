use std::cell::RefCell;
use std::rc::Rc;

use crate::{utils::iter_neighbors, Cell, CellContent, CellState, Coordinate, MineSweeper};

use super::Constraint;

pub(crate) const UNKNOWN: i32 = -5;
pub(crate) const CONSTRAINED: i32 = -4;
pub(crate) const MARKED: i32 = -3;
pub(crate) const MINE: i32 = -1;
pub(crate) const CLEAR: i32 = 0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct BoardCell {
    cell: Cell,
    pub coordinate: Coordinate,
    pub state: i32,
    boundary_level: i32,
    pub test_assignment: i32,
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

pub(crate) struct Board {
    pub unknown: i32,
    // constrained: i32,
    // mine: i32,
    // clear: i32,
    // unflagged_mines: i32,
    pub cells: Vec<Vec<Rc<RefCell<BoardCell>>>>,
}

impl Board {
    pub fn new(ms: &impl MineSweeper) -> Self {
        let mut cells = vec![Vec::with_capacity(ms.width()); ms.height()];
        for (r, row) in cells.iter_mut().enumerate() {
            for c in 0..ms.width() {
                row.push(Rc::new(RefCell::new(BoardCell::new(
                    ms.get_cell((r, c)).unwrap(),
                    (r, c),
                ))));
            }
        }
        Board {
            unknown: (ms.width() * ms.height()) as i32,
            // constrained: 0,
            // mine: 0,
            // clear: 0,
            // unflagged_mines: ms.mines(),
            cells,
        }
    }

    pub fn enumerate_boundary(&mut self, level: i32) -> Vec<Rc<RefCell<BoardCell>>> {
        // let mut result = Vec::with_capacity(self.unknown as i32);
        // for row in &self.cells {
        //     for cell in row {
        //         if cell.borrow().state == UNKNOWN && cell.borrow().boundary_level == level {
        //             result.push(Rc::clone(cell));
        //         }
        //     }
        // }
        // result
        self.cells
            .iter()
            .flat_map(|row| row.iter())
            .filter(|cell| cell.borrow().state == UNKNOWN && cell.borrow().boundary_level == level)
            .map(Rc::clone)
            .collect()
    }

    pub fn enumerate_max_boundary(&mut self) -> Vec<Rc<RefCell<BoardCell>>> {
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

    fn enumerate_min_boundary(&mut self) -> Vec<Rc<RefCell<BoardCell>>> {
        let mut min = i32::MAX;
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

    pub fn enumerate_unknown(&mut self) -> Vec<Rc<RefCell<BoardCell>>> {
        let mut result = Vec::with_capacity(self.unknown as usize);
        for row in &self.cells {
            for cell in row {
                if cell.borrow().state == UNKNOWN {
                    result.push(Rc::clone(cell));
                }
            }
        }
        result
    }

    fn enumerate_edges(&mut self) -> Vec<Rc<RefCell<BoardCell>>> {
        let mut result = Vec::new();
        for r in 1..(self.cells.len() - 1) {
            if self.cells[r][0].borrow().state < CONSTRAINED {
                result.push(Rc::clone(&self.cells[r][0]));
            }
            if self.cells[r][self.cells[r].len() - 1].borrow().state < CONSTRAINED {
                result.push(Rc::clone(&self.cells[r][self.cells[r].len() - 1]));
            }
        }
        for c in 1..(self.cells[0].len() - 1) {
            if self.cells[0][c].borrow().state < CONSTRAINED {
                result.push(Rc::clone(&self.cells[0][c]));
            }
            if self.cells[self.cells.len() - 1][c].borrow().state < CONSTRAINED {
                result.push(Rc::clone(&self.cells[self.cells.len() - 1][c]));
            }
        }
        result
    }

    pub fn enumerate_corners(&mut self) -> Vec<Rc<RefCell<BoardCell>>> {
        let mut result = Vec::with_capacity(4);
        if self.cells[0][0].borrow().state < CONSTRAINED {
            result.push(Rc::clone(&self.cells[0][0]));
        }
        if self.cells[0][self.cells[0].len() - 1].borrow().state < CONSTRAINED {
            result.push(Rc::clone(&self.cells[0][self.cells[0].len() - 1]));
        }
        if self.cells[self.cells.len() - 1][0].borrow().state < CONSTRAINED {
            result.push(Rc::clone(&self.cells[self.cells.len() - 1][0]));
        }
        if self.cells[self.cells.len() - 1][self.cells[0].len() - 1]
            .borrow()
            .state
            < CONSTRAINED
        {
            result.push(Rc::clone(
                &self.cells[self.cells.len() - 1][self.cells[0].len() - 1],
            ));
        }
        result
    }

    pub fn new_constraint(
        &mut self,
        coord @ (r, c): Coordinate,
    ) -> Option<Rc<RefCell<Constraint>>> {
        if self.cells[r][c].borrow().state < 0 {
            return None;
        }
        let mut state = self.cells[r][c].borrow().state;
        let mut constraint = Constraint::new();
        let mut constant = state;
        for (r, c) in iter_neighbors(coord, self.cells.len(), self.cells[0].len()).unwrap() {
            if self.cells[r][c].borrow().state < 0 {
                if self.cells[r][c].borrow().state == MARKED {
                    constant -= 1;
                } else {
                    constraint.add_variable(Rc::clone(&self.cells[r][c]));
                    self.set_state((r, c), CONSTRAINED);
                }
            }
        }
        constraint.set_constant(constant);
        Some(Rc::new(RefCell::new(constraint)))
    }

    fn open_cell(&mut self, (r, c): Coordinate) -> i32 {
        let state = self.cells[r][c].borrow().cell.state;
        match state {
            CellState::Flagged => MARKED,
            _ => {
                self.cells[r][c].borrow_mut().cell.state = CellState::Open;
                match self.cells[r][c].borrow().cell.content {
                    CellContent::Mine => MINE,
                    CellContent::Number(n) => n as i32,
                }
            }
        }
    }

    // in teoria non dovrebbe mai provare ad aprire una cella flaggata o già aperta (?)
    pub fn open(&mut self, coord: Coordinate) -> i32 {
        let result = self.open_cell(coord);
        self.set_state(coord, result);
        result
    }

    pub fn flag(&mut self, coord @ (r, c): Coordinate) {
        self.cells[r][c].borrow_mut().cell.state = CellState::Flagged;
        // self.unflagged_mines -= 1;
        self.set_state(coord, MARKED);
    }

    fn set_state(&mut self, coord @ (r, c): Coordinate, state: i32) {
        let mut cell = self.cells[r][c].borrow_mut();
        if cell.state == state {
            return;
        }
        // match cell.state {
        //     UNKNOWN => self.unknown -= 1,
        //     // CONSTRAINED => self.constrained -= 1,
        //     // MARKED => self.mine -= 1,
        //     _ => {
        //         // self.clear -= 1;
        //     }
        // }
        if cell.state == UNKNOWN {
            self.unknown -= 1
        }
        cell.state = state;
        match state {
            UNKNOWN => self.unknown += 1,
            CONSTRAINED => {
                // self.constrained += 1;
                cell.boundary_level = 0;
                for (r, c) in iter_neighbors(coord, self.cells.len(), self.cells[0].len()).unwrap()
                {
                    if self.cells[r][c].borrow().state == UNKNOWN {
                        self.cells[r][c].borrow_mut().boundary_level += 1;
                    }
                }
            }
            MARKED => {
                // self.mine += 1;
                cell.boundary_level = 0;
                for (r, c) in iter_neighbors(coord, self.cells.len(), self.cells[0].len()).unwrap()
                {
                    if self.cells[r][c].borrow().state == UNKNOWN {
                        self.cells[r][c].borrow_mut().boundary_level -= 1;
                    }
                }
            }
            _ => {
                cell.boundary_level = 0;
                // if state > 0 {
                //     self.clear += 1;
                // }
            }
        }
    }

    pub fn done(&self) -> bool {
        for i in 0..self.cells.len() {
            for j in 0..self.cells[i].len() {
                let cell = self.cells[i][j].borrow().cell;
                if cell.state != CellState::Open && cell.content != CellContent::Mine {
                    // if self.unknown == 0 {
                    //     eprintln!("Cannot use as condition")
                    // }
                    return false;
                }
            }
        }
        // if self.unknown != 0 {
        //     eprintln!("Cannot use as condition")
        // }
        true
    }

    pub fn unflagged_mines(&self) -> i32 {
        let mut result = 0;
        for i in 0..self.cells.len() {
            for j in 0..self.cells[i].len() {
                let cell = self.cells[i][j].borrow().cell;
                if cell.state != CellState::Flagged && cell.content == CellContent::Mine {
                    result += 1;
                }
            }
        }
        result
    }
}
