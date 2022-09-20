use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

use super::board::{Board, BoardCell, MARKED};

pub(crate) struct Constraint {
    variables: Vec<Rc<RefCell<BoardCell>>>,
    pub constant: i32,
    unassigned: i32,
    current_constant: i32,
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
        // self.variables.iter().map(Rc::clone).collect() // dovrebbe essere uguale
        self.variables.clone()
    }

    pub fn set_constant(&mut self, constant: i32) {
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
        self.current_constant <= self.constant
            && (self.unassigned > 0 || self.current_constant == self.constant)
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
        let mut state;
        for i in (0..self.variables.len()).rev() {
            state = <RefCell<_>>::borrow(&self.variables[i]).state;
            if state >= 0 {
                self.variables.swap_remove(i);
            } else if state == MARKED {
                self.variables.swap_remove(i);
                self.constant -= 1;
            }
        }
        let mut result;
        if self.is_empty() {
            return None;
        }
        if self.constant == 0 {
            result = Vec::with_capacity(self.variables.len());
            for variable in &self.variables {
                let coord = <RefCell<_>>::borrow(variable).coordinate;
                board.open(coord);
                if let Some(c) = board.new_constraint(coord) {
                    result.push(c);
                }
            }
        } else if self.constant == self.variables.len() as i32 {
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

    pub fn simplify(mut this: Rc<RefCell<Constraint>>, mut other: Rc<RefCell<Constraint>>) -> bool {
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

pub(crate) struct ConstraintList {
    pub variable: Rc<RefCell<BoardCell>>,
    pub constraints: Vec<Rc<RefCell<Constraint>>>,
}

impl ConstraintList {
    pub fn new(constraint: Rc<RefCell<Constraint>>, variable: Rc<RefCell<BoardCell>>) -> Self {
        ConstraintList {
            variable,
            constraints: vec![constraint],
        }
    }

    pub fn add_constraint(&mut self, constraint: Rc<RefCell<Constraint>>) {
        self.constraints.push(constraint);
    }

    pub fn update_constraints(&self) {
        for constraint in &self.constraints {
            <RefCell<_>>::borrow_mut(constraint).update_variable(Some(Rc::clone(&self.variable)));
        }
    }

    pub fn check_constraints(&self) -> bool {
        self.constraints.iter().all(|c| c.borrow().is_satisfied())
    }
}

impl PartialEq for ConstraintList {
    fn eq(&self, other: &Self) -> bool {
        self.constraints.len() == other.constraints.len()
    }
}

impl PartialOrd for ConstraintList {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.constraints
                .len()
                .cmp(&other.constraints.len())
                .reverse(),
        )
    }
}

impl Eq for ConstraintList {}

impl Ord for ConstraintList {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
