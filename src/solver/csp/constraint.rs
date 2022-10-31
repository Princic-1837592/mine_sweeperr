use std::borrow::BorrowMut;
use std::cell::RefCell;
#[cfg(test)]
use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;

use super::board::{Board, BoardCell, MARKED};

pub(crate) struct Constraint {
    variables: Vec<Rc<RefCell<BoardCell>>>,
    pub constant: isize,
    unassigned: isize,
    current_constant: isize,
    next_unassigned: Option<Rc<RefCell<BoardCell>>>,
}

impl Constraint {
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

    pub fn add_variable(&mut self, cell: Rc<RefCell<BoardCell>>) {
        self.variables.push(cell);
    }

    pub fn get_variables(&self) -> Vec<Rc<RefCell<BoardCell>>> {
        self.variables.iter().map(Rc::clone).collect()
    }

    pub fn set_constant(&mut self, constant: isize) {
        self.constant = constant;
    }

    pub fn update_variable(&mut self, _cell: Option<Rc<RefCell<BoardCell>>>) {
        self.current_constant = 0;
        self.unassigned = 0;
        self.next_unassigned = None;
        for variable in &self.variables {
            if <RefCell<_>>::borrow(variable).test_assignment < 0 {
                self.next_unassigned = Some(Rc::clone(variable));
                self.unassigned += 1;
            } else if <RefCell<_>>::borrow(variable).test_assignment >= 1 {
                self.current_constant += 1;
            }
        }
    }

    pub fn is_satisfied(&self) -> bool {
        if self.current_constant > self.constant {
            return false;
        }
        if self.unassigned > 0 {
            return true;
        }
        self.current_constant == self.constant
    }

    pub fn suggest_unassigned_variable(&self) -> Option<Rc<RefCell<BoardCell>>> {
        self.next_unassigned.as_ref()?;
        if self.current_constant == self.constant {
            <RefCell<_>>::borrow_mut(self.next_unassigned.as_ref().unwrap()).test_assignment = 0;
            return Some(Rc::clone(self.next_unassigned.as_ref().unwrap()));
        }
        if self.constant - self.current_constant == self.unassigned {
            <RefCell<_>>::borrow_mut(self.next_unassigned.as_ref().unwrap()).test_assignment = 1;
            return Some(Rc::clone(self.next_unassigned.as_ref().unwrap()));
        }
        None
    }

    pub fn update_and_remove_known_variables(
        &mut self,
        board: &mut Board,
    ) -> Option<Vec<Rc<RefCell<Constraint>>>> {
        for i in (0..self.variables.len()).rev() {
            let state = <RefCell<_>>::borrow(&self.variables[i]).state;
            if state >= 0 {
                self.variables.swap_remove(i);
            } else if state == MARKED {
                self.variables.swap_remove(i);
                self.constant -= 1;
            }
        }
        if self.is_empty() {
            return None;
        }

        let mut result;
        if self.constant == 0 {
            result = Vec::with_capacity(self.variables.len());
            for variable in &self.variables {
                let coord = <RefCell<_>>::borrow(variable).coordinate;
                board.open(coord);
                result.push(
                    board
                        .new_constraint(coord)
                        .expect("Unwrapping should be safe due to previous check"),
                );
            }
        } else if self.constant == self.variables.len() as isize {
            result = Vec::with_capacity(0);
            for variable in &self.variables {
                let coord = <RefCell<_>>::borrow(variable).coordinate;
                board.borrow_mut().flag(coord);
            }
        } else {
            return None;
        }

        self.variables.clear();
        self.constant = 0;
        Some(result)
    }

    pub fn simplify(this: Rc<RefCell<Constraint>>, other: Rc<RefCell<Constraint>>) -> bool {
        if this.borrow().variables.len() < other.borrow().variables.len() {
            return Constraint::simplify(other, this);
        }
        for i in 0..other.borrow().variables.len() {
            for j in 0..this.borrow().variables.len() {
                if this.borrow().variables[j] == other.borrow().variables[i] {
                    break;
                } else if j >= this.borrow().variables.len() - 1 {
                    return false;
                }
            }
        }
        let (o_len, t_len) = (
            other.borrow().variables.len(),
            this.borrow().variables.len(),
        );
        for i in 0..o_len {
            for j in 0..t_len {
                if this.borrow().variables[j] == other.borrow().variables[i] {
                    <RefCell<_>>::borrow_mut(&this).variables.swap_remove(j);
                    break;
                }
            }
        }
        <RefCell<_>>::borrow_mut(&this).constant -= other.borrow().constant;
        true
    }

    pub fn coupled_with(&mut self, other: Rc<RefCell<Constraint>>) -> bool {
        for i in 0..other.borrow().variables.len() {
            for j in 0..self.variables.len() {
                if self.variables[j] == other.borrow().variables[i] {
                    return true;
                }
            }
        }
        false
    }

    pub fn is_empty(&self) -> bool {
        self.variables.is_empty()
    }
}

#[cfg(test)]
impl Display for Constraint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            return write!(f, "[EMPTY CONSTRAINT]");
        }
        write!(f, "{} = ", self.constant)?;
        write!(
            f,
            "{}",
            self.variables
                .iter()
                .map(|c| c.borrow().to_string())
                .collect::<Vec<_>>()
                .join(" + ")
        )
    }
}

#[cfg(test)]
impl Debug for Constraint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
