mod mine_sweeper;
mod ms_hash;
mod ms_matrix;
mod utils;


pub use mine_sweeper::*;
pub use ms_hash::*;
pub use ms_matrix::*;


#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::{Cell, CellState, Error, MineSweeper};
    use crate::ms_matrix::MSMatrix;
    use crate::utils::iter_neighbors;


    #[test]
    fn it_works() {
        let (h, w, m) = (600, 10, 500);
        let mut ms: MSMatrix = MSMatrix::new(h, w, m).unwrap();
        println!("{}\n", ms);
        for i in 0..h {
            println!("{:?}", ms.open((i, 0)));
        }
        // assert_eq!(ms.open((w - 1, 0)), Err(Error::OutOfBounds));
        println!("{}\n", ms);
    }


    #[test]
    #[should_panic]
    fn it_panics() {
        let (h, w) = (10, 10);
        let m = w * h;
        MSMatrix::new(h, w, m - 1).unwrap();
        MSMatrix::new(h, w, m).unwrap();
    }


    #[test]
    fn new_cell_is_closed() {
        assert_eq!(Cell::default().state, CellState::Closed);
    }


    #[test]
    fn neighbors() {
        let (h, w) = (10, 10);
        let mut neighbors: HashSet<_> = iter_neighbors((0, 0), h, w).collect();
        println!("{:?}", neighbors);
        neighbors = iter_neighbors((h - 1, w - 1), h, w).collect();
        println!("{:?}", neighbors);
        neighbors = iter_neighbors((h - 1, w - 2), h, w).collect();
        println!("{:?}", neighbors);
        neighbors = iter_neighbors((0, 1), h, w).collect();
        println!("{:?}", neighbors);
        neighbors = iter_neighbors((1, 1), h, w).collect();
        println!("{:?}", neighbors);
    }
}
