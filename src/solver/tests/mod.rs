use std::fmt::Display;

use rand::{rngs::StdRng, thread_rng, Rng, SeedableRng};

use crate::{
    solver::{CSPSolver, NonDeterministic, SPSolver, Solver},
    Difficulty, MSMatrix, MineSweeper,
};

pub(crate) mod test_data;

#[test]
#[ignore]
fn solve() {
    fn test<'a, S, M>(boards: &[(usize, usize, &'a [usize])])
    where
        S: Solver,
        M: MineSweeper + Display + From<(usize, usize, &'a [usize])> + Clone,
    {
        let mut ms: M;
        let mut failed = false;
        let mut n_falied = 0;
        for (i, &board) in boards.iter().enumerate() {
            ms = board.into();
            if let Ok(false) = <S as Solver>::solve(&ms, (0, 0)) {
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

    // test::<SPSolver, MSMatrix>(&test_data::SP_SOLVABLE_EASY[..100]);
    // test::<CSPSolver, MSMatrix>(&test_data::CSP_SOLVABLE_HARD[..10]);
}

#[test]
fn generate() {
    fn test<S, M>(#[allow(unused)] seed: u64)
    where
        S: Solver,
        M: MineSweeper + Display + Clone,
        (usize, usize, std::vec::Vec<usize>, (usize, usize)): std::convert::From<M>,
    {
        // let mut rng = StdRng::seed_from_u64(seed);
        let mut rng = thread_rng();
        let ms = M::from_rng::<S>(Difficulty::custom(100, 100, 2000), (0, 0), &mut rng).unwrap();
    }
    for seed in 0..1 {
        test::<CSPSolver, MSMatrix>(seed);
    }
}
