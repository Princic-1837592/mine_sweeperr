use std::any::type_name;
use std::fmt::Display;

use rand::thread_rng;

use test_data::{MSFrom, CSP_SOLVABLE};

use crate::{
    solver::{CSPSolver, Solver},
    MSMatrix, MineSweeper,
};

pub mod test_data;

#[test]
fn solve() {
    fn test<'a, M, S>(boards: &'a [MSFrom<'a>])
    where
        M: MineSweeper + Display + From<MSFrom<'a>> + Clone,
        S: Solver<M>,
    {
        let mut ms: M;
        let mut failed = vec![];
        for (i, &board) in boards.iter().enumerate() {
            ms = board.into();
            if !<S>::new(&ms).solve(ms.started_from()) {
                failed.push(i);
                // println!("Failed to solve board {:?}", board);
            }
        }
        eprintln!(
            "{} failed to solve {}/{} ({:.2}%) boards: {}",
            type_name::<S>(),
            failed.len(),
            boards.len(),
            failed.len() as f64 / boards.len() as f64 * 100_f64,
            failed
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(", ")
        );
    }

    test::<MSMatrix, CSPSolver>(CSP_SOLVABLE);
}

#[test]
#[allow(unused)]
fn generate() {
    fn test<M, S>(seed: u64) {
        // let mut rng = StdRng::seed_from_u64(seed);
        let mut rng = thread_rng();
    }

    for seed in 0..10 {
        test::<MSMatrix, CSPSolver>(seed);
    }
}
