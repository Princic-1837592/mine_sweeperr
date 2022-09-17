use std::fmt::Display;

use rand::{rngs::StdRng, thread_rng, Rng, SeedableRng};

use crate::{
    solver::{CSPSolver, NonDeterministic, SPSolver, Solver},
    Difficulty, MSMatrix, MineSweeper,
};

mod test_data;

#[test]
#[ignore]
fn solve() {
    fn test<'a, S, M>(boards: &[(usize, usize, &'a [usize])])
    where
        S: Solver<M>,
        M: MineSweeper + Display + From<(usize, usize, &'a [usize])>,
    {
        let mut ms: M;
        let mut failed = false;
        let mut n_falied = 0;
        for (i, &board) in boards.iter().enumerate() {
            ms = board.into();
            if let Ok(false) = <S as Solver<_>>::solve(ms, (0, 0)) {
                // eprintln!("Solver failed to solve {}th board: {:?}", i, board);
                n_falied += 1;
                failed = true;
            }
        }
        println!(
            "Solver failed to solve {}/{} boards",
            n_falied,
            boards.len()
        );
        // assert!(!failed);
    }

    test::<SPSolver, MSMatrix>(&test_data::SP_SOLVABLE_EASY[..100]);
    test::<CSPSolver, MSMatrix>(&test_data::CSP_SOLVABLE_HARD[..10]);
}
