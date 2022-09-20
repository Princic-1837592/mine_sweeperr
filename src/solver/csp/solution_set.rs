use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::rc::Rc;

use crate::solver::csp::board::{Board, BoardCell};
use crate::solver::csp::constraint::{Constraint, ConstraintList};

pub(crate) struct SolutionSet {
    constraints: Vec<Rc<RefCell<Constraint>>>,
    variables: Vec<Rc<RefCell<BoardCell>>>,
    nodes: Vec<ConstraintList>,
    solutions: Vec<i32>,
    mines: Vec<Vec<i32>>,
    min: i32,
    max: i32,
}

impl SolutionSet {
    pub fn new(constraints: Vec<Rc<RefCell<Constraint>>>) -> Self {
        let nodes = Vec::with_capacity(constraints.len() * 2);
        let mut result = SolutionSet {
            constraints,
            variables: Vec::new(),
            nodes,
            solutions: Vec::new(),
            mines: Vec::new(),
            min: 0,
            max: 0,
        };
        result.construct();
        result
    }

    fn construct(&mut self) {
        for constraint in &self.constraints {
            let (mut var_array, mut found);
            var_array = <RefCell<_>>::borrow(constraint).get_variables();
            for variable in var_array {
                found = false;
                for node in self.nodes.iter_mut() {
                    if node.variable == variable {
                        node.add_constraint(Rc::clone(constraint));
                        found = true;
                        break;
                    }
                }
                if !found {
                    self.nodes.push(ConstraintList::new(
                        Rc::clone(constraint),
                        Rc::clone(&variable),
                    ));
                }
            }
            self.min += <RefCell<_>>::borrow(constraint).constant;
        }
        self.nodes.sort();
        self.variables.reserve(self.nodes.len());
        self.variables
            .extend(self.nodes.iter().map(|node| Rc::clone(&node.variable)));
        if self.min < 0 {
            println!("Initializing with {}", self.min);
        }
        self.solutions = vec![0; (self.min + 1) as usize];
        self.mines.reserve((self.min + 1) as usize);
        for _ in 0..self.min + 1 {
            self.mines.push(vec![0; self.variables.len()]);
        }
    }

    pub fn get_variable_count(&self) -> i32 {
        self.variables.len() as i32
    }

    pub fn get_constraint_count(&self) -> i32 {
        self.constraints.len() as i32
    }

    pub fn get_min(&self) -> i32 {
        self.min
    }

    pub fn get_max(&self) -> i32 {
        self.max
    }

    pub fn expected_mines(&self) -> f32 {
        let (mut total, mut count) = (0, 0);
        for i in self.min..self.max + 1 {
            total += i * self.solutions[i as usize];
            count += self.solutions[i as usize];
        }
        total as f32 / count as f32
    }

    pub fn reduce_min_max(&mut self, min: i32, max: i32) {
        if min > self.min {
            for i in self.min..min {
                self.solutions[i as usize] = 0;
            }
            self.min = min;
        }
        if max < self.max {
            for i in (max + 1..=self.max).rev() {
                self.solutions[i as usize] = 0;
            }
            self.max = max;
        }
    }

    pub fn mark_mines(&self, board: &mut Board) {
        let mut total_solutions = 0;
        for j in self.min..=self.max {
            total_solutions += self.solutions[j as usize];
        }
        for i in 0..self.variables.len() {
            let mut total = 0;
            for j in self.min..=self.max {
                total += self.mines[j as usize][i];
            }
            if total == total_solutions {
                let coord = <RefCell<_>>::borrow(&self.variables[i]).coordinate;
                board.flag(coord);
            }
        }
    }

    pub fn enumerate_solutions(&mut self) {
        for i in 0..self.solutions.len() {
            self.solutions[i] = 0;
            for j in 0..self.variables.len() {
                self.mines[i][j] = 0;
            }
        }
        for variable in &self.variables {
            <RefCell<_>>::borrow_mut(variable).test_assignment = -1;
        }
        let mut variable_index = vec![-1; self.variables.len()];
        let mut last_choice = -1;
        for constraint in &self.constraints {
            <RefCell<_>>::borrow_mut(constraint).update_variable(None);
        }
        let mut level = 0;
        loop {
            if level == self.variables.len() as i32 {
                let mut m = 0;
                for variable in &self.variables {
                    m += <RefCell<_>>::borrow(variable).test_assignment;
                }
                self.solutions[m as usize] += 1;
                if m < self.min {
                    self.min = m;
                }
                if m > self.max {
                    self.max = m;
                }
                for (j, variable) in self.variables.iter().enumerate() {
                    self.mines[m as usize][j] += <RefCell<_>>::borrow(variable).test_assignment;
                }
                level -= 1;
                continue;
            }

            if variable_index[level as usize] < 0 {
                let mut variable = None;
                let mut i = 0;
                while variable == None && i < self.constraints.len() {
                    variable =
                        <RefCell<_>>::borrow(&self.constraints[i]).suggest_unassigned_variable();
                    i += 1;
                }
                match variable {
                    Some(mut var) => {
                        variable_index[level as usize] = self.variables.len() as i32;
                        loop {
                            variable_index[level as usize] -= 1;
                            if self.variables[variable_index[level as usize] as usize] == var {
                                break;
                            }
                        }
                        <RefCell<_>>::borrow_mut(&var).test_assignment -= 1;
                    }
                    None => {
                        loop {
                            last_choice += 1;
                            if !<RefCell<_>>::borrow(&self.variables[last_choice as usize])
                                .test_assignment
                                >= 0
                            {
                                break;
                            }
                        }
                        variable_index[level as usize] = last_choice;
                    }
                }
            }

            if <RefCell<_>>::borrow(&self.variables[variable_index[level as usize] as usize])
                .test_assignment
                > 0
            {
                if variable_index[level as usize] <= last_choice {
                    last_choice = variable_index[level as usize] - 1;
                }
                <RefCell<_>>::borrow_mut(
                    &self.variables[variable_index[level as usize] as usize],
                )
                .test_assignment = -1;
                self.nodes[variable_index[level as usize] as usize]
                    .borrow()
                    .update_constraints();
                variable_index[level as usize] = -1;
                level -= 1;
            } else {
                <RefCell<_>>::borrow_mut(
                    &self.variables[variable_index[level as usize] as usize],
                )
                .test_assignment += 1;
                self.nodes[variable_index[level as usize] as usize]
                    .borrow()
                    .update_constraints();
                if self.nodes[variable_index[level as usize] as usize].check_constraints() {
                    level += 1;
                }
            }

            if !level >= 0 {
                break;
            }
        }
    }
}
