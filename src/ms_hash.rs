use std::collections::HashSet;
use crate::mine_sweeper::Coordinate;


/// **TODO: NOT IMPLEMENTED YET**
///
/// Represents a grid using [`HashSets`](HashSet) of [`Coordinates`](Coordinate).
/// Use this when you don't want to load the whole grid in memory at the beginning.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MSHash {
    width: usize,
    height: usize,
    open: HashSet<Coordinate>,
    flagged: HashSet<Coordinate>,
    mines: HashSet<Coordinate>,
}
