use std::{cell::RefCell, rc::Rc};

use board::Board;
use constraint::Constraint;

use super::{csp::solution_set::SolutionSet, Solver};
use crate::{solver::csp::board::MINE, Coordinate, MineSweeper};

mod board;
mod constraint;
mod solution_set;
#[cfg(test)]
mod tests;

pub struct CSPSolver {
    constraints: Vec<Rc<RefCell<Constraint>>>,
    board: Board,
}

impl CSPSolver {
    fn solve(&mut self, start_from: Coordinate) -> bool {
        if self.board.open(start_from) == MINE {
            return false;
        }
        for i in 0..self.board.cells.len() {
            for j in 0..self.board.cells[i].len() {
                if let Some(constraint) = self.board.new_constraint((i, j)) {
                    self.constraints.push(constraint);
                }
            }
        }
        while !self.board.done() {
            self.simplify_constraints();
            if self.board.done() {
                break;
            }
            let mut subsets = self.separate_constraints();
            if !subsets.is_empty() {
                for subset in &mut subsets {
                    subset.enumerate_solutions();
                }
            }
            let remaining = self.board.unflagged_mines();
            let far = self.board.unknown;
            let mut far_max = remaining as isize;
            for i in 0..subsets.len() {
                let (mut min, mut max) = (0, far as isize);
                for (j, subset) in subsets.iter().enumerate() {
                    if i != j {
                        min += subset.get_min();
                        max += subset.get_max();
                    }
                }
                subsets[i].reduce_min_max(remaining - max, remaining - min);
                far_max -= subsets[i].get_min() as isize;
            }
            for subset in subsets {
                subset.mark_mines(&mut self.board);
            }
            if far_max <= 0 && far > 0 {
                let positions = self.board.enumerate_unknown();
                for coordinate in positions.iter().map(|x| x.borrow().coordinate) {
                    self.board.open(coordinate);
                    if let Some(constraint) = self.board.new_constraint(coordinate) {
                        self.constraints.push(constraint);
                    }
                }
                continue;
            }
            break;
        }
        self.board.done()
    }

    fn separate_constraints(&mut self) -> Vec<SolutionSet> {
        let mut result = Vec::new();
        let mut start = 0;
        for end in 1..=self.constraints.len() {
            let mut found = false;
            for i in end..self.constraints.len() {
                if found {
                    break;
                }
                for j in start..end {
                    if <RefCell<_>>::borrow_mut(&self.constraints[i])
                        .coupled_with(Rc::clone(&self.constraints[j]))
                    {
                        found = true;
                        if i != end {
                            self.constraints.swap(i, end);
                        }
                        break;
                    }
                }
            }
            if !found {
                // ATTENZIONE: il passaggio del vettore si può fare anche così:
                // self.constraints[start..end].to_vec()
                // ma questo metodo copia le Rc<RefCell<Constraint>> implicitamente,
                // in contrapposizione all'utilizzo di Rc::clone nel resto del codice
                result.push(SolutionSet::new(
                    self.constraints[start..end].iter().map(Rc::clone).collect(),
                ));
                start = end;
            }
        }
        result
    }

    fn simplify_constraints(&mut self) {
        loop {
            let mut done = true;
            let mut i = 0;
            loop {
                let mut to_extend = Vec::new();
                while i < self.constraints.len() {
                    if let Some(new_constraints) = <RefCell<_>>::borrow_mut(&self.constraints[i])
                        .update_and_remove_known_variables(&mut self.board)
                    {
                        done = false;
                        to_extend.extend(new_constraints);
                    }
                    i += 1;
                }
                if to_extend.is_empty() {
                    break;
                }
                self.constraints.extend(to_extend);
            }
            if !done {
                continue;
            }
            let mut i = 0;
            while i < self.constraints.len() {
                while i < self.constraints.len()
                    && <RefCell<_>>::borrow(&self.constraints[i]).is_empty()
                {
                    self.constraints.swap_remove(i);
                }
                if i < self.constraints.len() {
                    for j in i + 1..self.constraints.len() {
                        if Constraint::simplify(
                            Rc::clone(&self.constraints[i]),
                            Rc::clone(&self.constraints[j]),
                        ) {
                            done = false;
                        }
                    }
                }
                i += 1;
            }
            if done {
                break;
            }
        }
    }
}

impl<M: MineSweeper> Solver<M> for CSPSolver {
    fn new(ms: &M) -> Self {
        let board = Board::new(ms);
        CSPSolver {
            constraints: Vec::with_capacity(ms.width() * ms.height()),
            board,
        }
    }

    fn solve(&mut self, start_from: Coordinate) -> bool {
        let result = self.solve(start_from);
        // #[cfg(test)]
        // {
        //     // println!(
        //     //     "{}",
        //     //     this.constraints
        //     //         .iter()
        //     //         .map(|r| r.borrow().to_string())
        //     //         .collect::<Vec<_>>()
        //     //         .join("\n")
        //     // );
        //     let sets = this.separate_constraints();
        //     println!(
        //         "[{}]",
        //         sets.iter()
        //             .map(ToString::to_string)
        //             .collect::<Vec<_>>()
        //             .join(",\n\n")
        //     );
        // }
        result
        // if result {
        //     Ok(())
        // } else {
        //     Err(this
        //         .separate_constraints()
        //         .iter()
        //         .map(|set| {
        //             let mut result: Vec<_> = set
        //                 .get_variables()
        //                 .iter()
        //                 .map(|v| v.borrow().coordinate)
        //                 .collect();
        //             result.sort();
        //             result.dedup();
        //             result
        //         })
        //         .collect())
        // }
    }
}
