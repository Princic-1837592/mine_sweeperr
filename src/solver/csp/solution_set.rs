#[cfg(test)]
use std::fmt::{Debug, Display, Formatter};
use std::{borrow::Borrow, cell::RefCell, cmp::Ordering, rc::Rc};

use crate::solver::csp::{
    board::{Board, BoardCell},
    constraint::Constraint,
};

pub(crate) struct SolutionSet {
    constraints: Vec<Rc<RefCell<Constraint>>>,
    variables: Vec<Rc<RefCell<BoardCell>>>,
    nodes: Vec<ConstraintList>,
    solutions: Vec<isize>,
    mines: Vec<Vec<isize>>,
    min: isize,
    max: isize,
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
            let (var_array, mut found);
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

    #[allow(unused)]
    pub fn get_variable_count(&self) -> isize {
        self.variables.len() as isize
    }

    #[allow(unused)]
    pub fn get_constraint_count(&self) -> isize {
        self.constraints.len() as isize
    }

    pub fn get_min(&self) -> isize {
        self.min
    }

    pub fn get_max(&self) -> isize {
        self.max
    }

    #[allow(unused)]
    pub fn expected_mines(&self) -> f32 {
        let (mut total, mut count) = (0, 0);
        for i in self.min..self.max + 1 {
            total += i * self.solutions[i as usize];
            count += self.solutions[i as usize];
        }
        total as f32 / count as f32
    }

    pub fn reduce_min_max(&mut self, min: isize, max: isize) {
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
            if level == self.variables.len() as isize {
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
                    Some(var) => {
                        variable_index[level as usize] = self.variables.len() as isize;
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

    pub fn get_variables(&self) -> Vec<Rc<RefCell<BoardCell>>> {
        self.variables.iter().map(Rc::clone).collect()
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
        self.constraints
            .iter()
            .all(|c| <RefCell<_>>::borrow(c).is_satisfied())
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

#[cfg(test)]
impl Debug for ConstraintList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.constraints
                .iter()
                .map(|c| <RefCell<_>>::borrow(c).to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

#[cfg(test)]
impl Display for ConstraintList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
impl Debug for SolutionSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}]",
            self.nodes
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(",\n")
        )
    }
}

#[cfg(test)]
impl Display for SolutionSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
