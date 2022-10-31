pub use csp::CSPSolver;
pub use single_point::SPSolver;

use crate::{Coordinate, MineSweeper};

mod csp;
mod single_point;

#[cfg(test)]
mod tests;

/// This trait represents a minimal interface to write solvers
/// applied to the [`MineSweeper`](MineSweeper) game.
/// A solver can be any strategy applied to a game that tries to solve it (even randomly).
pub trait Solver<M: MineSweeper> {
    fn new(ms: &M) -> Self;
    /// Apply the implemented strategy to a [`MineSweeper`](MineSweeper) game.
    /// Returns `true` if the board can be solved by the strategy, `false` otherwise.
    /// This method should be able to safely assume that the given coordinate is valid.
    fn solve(&mut self, start_from: Coordinate) -> bool;
    /// Use this after a call to [`solve`](Solver::solve) to get the number of times the strategy
    /// had to guess a move due to not enough information.
    /// Returning `0` should mean that the strategy is able to solve the board perfectly,
    /// and the same should be for a player applying the same strategy.
    ///
    /// # Default
    /// The default implementation returns [`usize::MAX`](usize::MAX).
    fn guessed() -> usize {
        usize::MAX
    }
    /// Use this after a call to [`solve`](Solver::solve).
    /// This method should return an array of **unsolvable clusters** left in the given board.
    /// Unsolvable clusters are groups of cells that the strategy couldn't solve.
    /// This can be useful to implement a system in which a [`MineSweeper`](MineSweeper) implementor
    /// tries to solve itself, gets response about what's left unsolved,
    /// makes some adjustments and then tries to solve itself again.
    ///
    /// # Default
    /// The default implementation returns an empty vector.
    fn get_unsolvable_clusters() -> Vec<Vec<Coordinate>> {
        Vec::new()
    }
}

pub struct NonDeterministic {}

impl<M: MineSweeper> Solver<M> for NonDeterministic {
    fn new(_: &M) -> Self {
        Self {}
    }

    fn solve(&mut self, _: Coordinate) -> bool {
        true
    }
}
