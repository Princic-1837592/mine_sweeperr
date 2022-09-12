use crate::{
    solver::{CSPSolver, NonDeterministic, SPSolver, Solver},
    Difficulty, MSMatrix, MineSweeper,
};
use rand::{rngs::StdRng, thread_rng, Rng, SeedableRng};
use std::fmt::Display;

#[test]
// #[ignore]
fn solve() {
    fn test<S, M>(_seed: u64)
    where
        S: Solver<M>,
        M: MineSweeper + Display,
    {
        let mut rng = StdRng::seed_from_u64(_seed);
        // let mut rng = thread_rng();

        let difficulty = Difficulty::easy();
        let (h, w, _) = difficulty.into();
        let start_from = (rng.gen_range(0..h), rng.gen_range(0..w));
        let mut ms = M::from_rng::<S, _>(difficulty, start_from, &mut rng).unwrap();
    }

    for seed in 0..1 {
        test::<SPSolver, MSMatrix>(seed);
    }
}
