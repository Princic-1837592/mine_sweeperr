use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

use crate::{Cell, CellContent, CellState, Coordinate, iter_neighbors, MineSweeper};

use super::Constraint;


pub(crate) const UNKNOWN: isize = -5;
pub(crate) const CONSTRAINED: isize = -4;
pub(crate) const MARKED: isize = -3;
pub(crate) const MINE: isize = -1;
#[allow(unused)]
pub(crate) const CLEAR: isize = 0;

#[derive(Debug, Clone, Copy)]
pub(crate) struct BoardCell {
    cell: Cell,
    pub coordinate: Coordinate,
    pub state: isize,
    boundary_level: isize,
    pub test_assignment: isize,
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

impl Display for BoardCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}{}, {})",
            match self.state {
                UNKNOWN => "U".to_string(),
                CONSTRAINED => "C".to_string(),
                MARKED => "M".to_string(),
                MINE => "X".to_string(),
                CLEAR => " ".to_string(),
                n => n.to_string(),
            },
            self.coordinate.0,
            self.coordinate.1,
        )
    }
}

pub(crate) struct Board {
    pub unknown: usize,
    // constrained: usize,
    // mine: usize,
    clear: usize,
    unflagged_mines: usize,
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
            unknown: (ms.width() * ms.height()),
            // constrained: 0,
            // mine: 0,
            clear: 0,
            unflagged_mines: ms.mines(),
            cells,
        }
    }

    pub fn enumerate_boundary(&mut self, level: isize) -> Vec<Rc<RefCell<BoardCell>>> {
        // let mut result = Vec::with_capacity(self.unknown as isize);
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

    #[allow(unused)]
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

    #[allow(unused)]
    fn enumerate_min_boundary(&mut self) -> Vec<Rc<RefCell<BoardCell>>> {
        let mut min = isize::MAX;
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
        let mut result = Vec::with_capacity(self.unknown);
        for row in &self.cells {
            for cell in row {
                if cell.borrow().state == UNKNOWN {
                    result.push(Rc::clone(cell));
                }
            }
        }
        result
    }

    #[allow(unused)]
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

    #[allow(unused)]
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
        let mut constraint = Constraint::new();
        let mut constant = self.cells[r][c].borrow().state;
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

    fn open_cell(&mut self, (r, c): Coordinate) -> isize {
        let state = self.cells[r][c].borrow().cell.state;
        match state {
            CellState::Flagged => MARKED,
            _ => {
                self.cells[r][c].borrow_mut().cell.state = CellState::Open;
                match self.cells[r][c].borrow().cell.content {
                    CellContent::Mine => MINE,
                    CellContent::Number(n) => n as isize,
                }
            }
        }
    }

    // in teoria non dovrebbe mai provare ad aprire una cella flaggata o già aperta (?)
    pub fn open(&mut self, coord: Coordinate) -> isize {
        let result = self.open_cell(coord);
        self.set_state(coord, result);
        result
    }

    pub fn flag(&mut self, coord @ (r, c): Coordinate) {
        self.cells[r][c].borrow_mut().cell.state = CellState::Flagged;
        self.unflagged_mines -= 1;
        self.set_state(coord, MARKED);
    }

    fn set_state(&mut self, coord @ (r, c): Coordinate, state: isize) {
        let mut cell = self.cells[r][c].borrow_mut();
        if cell.state == state {
            return;
        }
        match cell.state {
            UNKNOWN => self.unknown -= 1,
            // CONSTRAINED => self.constrained -= 1,
            // MARKED => self.mine -= 1,
            CONSTRAINED | MARKED => {}
            _ => {
                self.clear -= 1;
            }
        }
        // if cell.state == UNKNOWN {
        //     self.unknown -= 1
        // }
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
                if state >= 0 {
                    self.clear += 1;
                }
            }
        }
    }

    pub fn done(&self) -> bool {
        for i in 0..self.cells.len() {
            for j in 0..self.cells[i].len() {
                let cell = self.cells[i][j].borrow().cell;
                if cell.state != CellState::Open && cell.content != CellContent::Mine {
                    // if self.unknown == 0 {
                    //     eprintln!(
                    //         "Cannot use as condition: {} == 0 but shouldn't ({})",
                    //         self.unknown, self.clear
                    //     );
                    // }
                    return false;
                }
            }
        }
        // if self.unknown != 0 {
        //     eprintln!(
        //         "Cannot use as condition: {} != 0 but shouldn't ({})",
        //         self.unknown, self.clear
        //     )
        // }
        true
    }

    pub fn unflagged_mines(&self) -> isize {
        self.unflagged_mines as isize
        // let mut result = 0;
        // for i in 0..self.cells.len() {
        //     for j in 0..self.cells[i].len() {
        //         let cell = self.cells[i][j].borrow().cell;
        //         if cell.state != CellState::Flagged && cell.content == CellContent::Mine {
        //             result += 1;
        //         }
        //     }
        // }
        // result
    }
}

impl PartialEq for BoardCell{
    fn eq(&self, other: &Self) -> bool {
        self.coordinate == other.coordinate
    }
}

impl Eq for BoardCell {}
