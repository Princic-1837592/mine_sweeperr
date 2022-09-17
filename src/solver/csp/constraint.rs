use std::borrow::{Borrow, BorrowMut};
use std::cell::{Ref, RefCell};

use crate::{Coordinate, MineSweeper};

use super::board::{Board, BoardCell, MARKED};

pub(crate) struct Constraint<'a> {
    pub variables: Vec<&'a RefCell<BoardCell>>,
    // nvariables: u8,
    pub constant: u8,
    pub unassigned: u8,
    pub current_constant: u8,
    pub next_unassigned: Option<&'a RefCell<BoardCell>>,
}

impl<'a> Constraint<'a> {
    pub fn new() -> Self {
        Constraint {
            variables: Vec::with_capacity(8),
            constant: 0,
            // nvariables: 0,
            unassigned: 0,
            current_constant: 0,
            next_unassigned: None,
        }
    }

    pub fn add_variable(&mut self, cell: &'a RefCell<BoardCell>) {
        self.variables.push(cell);
    }

    pub fn set_constant(&mut self, constant: u8) {
        self.constant = constant;
    }

    pub fn update_variable(&mut self, cell: &'a RefCell<BoardCell>) {
        self.current_constant = 0;
        self.unassigned = 0;
        self.next_unassigned = None;
        for &variable in &self.variables {
            if variable.borrow().test_assignment < 0 {
                self.next_unassigned = Some(variable);
                self.unassigned += 1;
            } else if variable.borrow().test_assignment >= 1 {
                self.current_constant += 1;
            }
        }
    }

    pub fn is_satisfied(&self) -> bool {
        self.unassigned > 0 || self.current_constant == self.constant
    }

    pub fn suggest_unassigned_variable(&self) -> Option<&'a RefCell<BoardCell>> {
        match self.next_unassigned {
            None => None,
            Some(next) => {
                if self.current_constant == self.constant {
                    next.borrow_mut().test_assignment = 0;
                    Some(next)
                } else if self.constant - self.current_constant == self.unassigned {
                    next.borrow_mut().test_assignment = 1;
                    Some(next)
                } else {
                    None
                }
            }
        }
    }

    pub fn update_and_remove_known_variables(
        &'a mut self,
        board: RefCell<Board<'a>>,
    ) -> Option<Vec<Constraint>> {
        let mut state;
        for i in (0..self.variables.len()).rev() {
            state = self.variables[i].borrow().state;
            if state >= 0 {
                self.variables.swap_remove(i);
            } else if state == MARKED {
                self.variables.swap_remove(i);
                self.constant -= 1;
            }
        }
        let mut result: Vec<Constraint>;
        if self.variables.is_empty() {
            return None;
        }
        if self.constant == 0 {
            result = Vec::with_capacity(self.variables.len());
            for &variable in &self.variables {
                let coord = variable.borrow().coordinate;
                board.borrow_mut().open(coord);
                if let Some(c) = board.borrow().new_constraint(coord) {
                    result.push(c);
                }
            }
        } else if self.constant == self.variables.len() as u8 {
            result = Vec::with_capacity(0);
            for &variable in &self.variables {
                board.borrow_mut().flag(variable.borrow().coordinate);
            }
        } else {
            return None;
        }

        self.variables.clear();
        self.constant = 0;
        Some(result)
    }

    pub fn simplify(&mut self, other: &mut Constraint) -> bool {
        if self.variables.len() < other.variables.len() {
            return other.simplify(self);
        }
        for i in 0..other.variables.len() {
            for j in 0..self.variables.len() {
                if self.variables[j] == other.variables[i] {
                    break;
                } else if j >= self.variables.len() - 1 {
                    return false;
                }
            }
        }
        for i in 0..other.variables.len() {
            for j in 0..self.variables.len() {
                if self.variables[j] == other.variables[i] {
                    self.variables.swap_remove(j);
                    break;
                }
            }
        }
        self.constant -= other.constant;
        true
    }

    pub fn coupled_with(&mut self, other: &mut Constraint) -> bool {
        for i in 0..other.variables.len() {
            for j in 0..self.variables.len() {
                if self.variables[j] == other.variables[i] {
                    return true;
                }
            }
        }
        false
    }
}
