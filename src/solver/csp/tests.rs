use rand::rngs::StdRng;
use rand::SeedableRng;

use crate::{Difficulty, MSMatrix, MineSweeper, Solver};

use super::super::tests::test_data;
use super::CSPSolver;

#[test]
fn easy_game() {
    let mut rng = StdRng::seed_from_u64(0);
    // let ms = MSMatrix::from_rng::<CSPSolver>(Difficulty::medium(), (0, 0), &mut rng).unwrap();
    let ms = MSMatrix::from((
        16,
        30,
        &[
            2_usize, 5, 7, 8, 9, 13, 17, 19, 21, 29, 34, 36, 45, 49, 55, 56, 58, 66, 72, 75, 76,
            86, 89, 94, 101, 104, 107, 112, 113, 120, 128, 137, 140, 156, 163, 165, 169, 174, 187,
            191, 194, 198, 199, 210, 215, 228, 234, 239, 245, 252, 266, 267, 269, 270, 272, 273,
            280, 293, 294, 310, 315, 319, 331, 333, 339, 343, 344, 349, 350, 352, 353, 357, 358,
            363, 366, 376, 381, 382, 389, 390, 395, 396, 398, 409, 414, 416, 420, 425, 432, 434,
            436, 444, 449, 455, 461, 466, 471, 473, 478,
        ] as &[usize],
        (5, 4),
    ));
    // for i in 0..ms.height() {
    //     for j in 0..ms.width() {
    //         print!("{:<10} ", ms.get_cell((i, j)).unwrap().content);
    //     }
    //     println!();
    // }
    let s = <CSPSolver as Solver>::solve(&ms, ms.started_from()).unwrap();
    println!("{}", s);
}
