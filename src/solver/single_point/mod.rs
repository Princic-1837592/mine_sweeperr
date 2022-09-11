use crate::{Coordinate, MineSweeper, Solver};

struct SPSolver {}

impl SPSolver {
    fn apply(ms: &mut impl MineSweeper, (r, c): Coordinate) -> ! {
        todo!()
    }

    fn unknowns_near(ms: &impl MineSweeper, (r, c): Coordinate) -> ! {
        todo!()
    }

    fn marks_near(ms: &impl MineSweeper, (r, c): Coordinate) -> ! {
        todo!()
    }

    fn probe_around(ms: &impl MineSweeper, (r, c): Coordinate) -> ! {
        todo!()
    }

    fn mark_around(ms: &impl MineSweeper, (r, c): Coordinate) -> ! {
        todo!()
    }

    fn adjoin_around(ms: &impl MineSweeper, (r, c): Coordinate) -> ! {
        todo!()
    }
}

impl<M: MineSweeper> Solver<M> for SPSolver {
    fn new() -> Self {
        todo!()
    }

    fn solve(ms: M, start_from: Coordinate) -> crate::Result<bool> {
        loop {}
    }
}
