use rand::rngs::StdRng;
use rand::SeedableRng;

use crate::solver::SPSolver;
use crate::{Difficulty, MSMatrix, MineSweeper, Solver};

#[test]
#[ignore]
fn easy_game() {
    // let mut rng = StdRng::seed_from_u64(0);
    let ms = MSMatrix::from((
        9,
        9,
        &[
            12_usize, 16, 17, 18, 35, 42, 47, 52, 74, 78, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13,
            14, 15,
        ] as &[usize],
        (0, 0),
    ));
    for i in 0..ms.height() {
        for j in 0..ms.width() {
            print!("{:<10} ", ms.get_cell((i, j)).unwrap().content);
        }
        println!();
    }
    assert!(SPSolver::solve(&ms, (4, 4)).unwrap());
}
