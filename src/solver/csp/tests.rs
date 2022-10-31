use crate::solver::CSPSolver;
use crate::{MSMatrix, MineSweeper, Solver};

use super::super::tests::test_data::CSP_SOLVABLE;

#[test]
#[allow(unused)]
#[ignore]
fn test() {
    let board = CSP_SOLVABLE[67];
    let ms: MSMatrix = board.into();
    let mut solver = CSPSolver::new(&ms);
    solver.solve(ms.started_from());
}
