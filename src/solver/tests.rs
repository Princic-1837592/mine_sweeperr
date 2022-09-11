use crate::{
    solver::{CSPSolver, NonDeterministic, SPSolver, Solver},
    Difficulty, MSMatrix, MineSweeper,
};
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::fmt::Display;

#[test]
// todo non ha senso testare csp su una partita generata deterministicamente con csp
// avrebbe senso implementare il SP e testare il CSP su una partita deterministica
fn solve() {
    fn test<S, M>(_seed: u64)
    where
        S: Solver<M>,
        M: MineSweeper + Display,
    {
        let mut rng = StdRng::seed_from_u64(_seed);
        // let mut rng = thread_rng();

        let difficulty = Difficulty::custom(5, 5, 3);
        let (h, w, _) = difficulty.into();
        let start_from = (rng.gen_range(0..h), rng.gen_range(0..w));
        let mut ms = M::from_rng::<S, _>(difficulty, start_from, &mut rng).unwrap();
        println!("{}", ms);
        // assert!(
        //     S::solve(&mut ms, (0, 0)).unwrap(),
        //     "CSP solver should solve a deterministic board"
        // );
    }

    for seed in 0..10 {
        test::<SPSolver, MSMatrix>(seed);
    }
}
