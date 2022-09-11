use crate::{
    solver::{CSPSolver, NonDeterministic, Solver},
    Difficulty, MSMatrix, MineSweeper,
};
use rand::{rngs::StdRng, Rng, SeedableRng};

#[test]
#[ignore]
// todo non ha senso testare csp su una partita generata deterministicamente con csp
// avrebbe senso implementare il SP e testare il CSP su una partita deterministica
fn solve() {
    fn test<S, M>(_seed: u64)
    where
        S: Solver<M>,
        M: MineSweeper,
    {
        let mut rng = StdRng::seed_from_u64(_seed);
        // let mut rng = thread_rng();

        let difficulty = Difficulty::medium();
        let (h, w, _) = difficulty.into();
        let start_from = (rng.gen_range(0..h), rng.gen_range(0..w));
        let mut ms = M::from_rng::<NonDeterministic, _>(difficulty, start_from, &mut rng).unwrap();
        // assert!(
        //     S::solve(&mut ms, (0, 0)).unwrap(),
        //     "CSP solver should solve a deterministic board"
        // );
    }

    for seed in 0..10 {
        test::<CSPSolver<MSMatrix>, _>(seed);
    }
}
