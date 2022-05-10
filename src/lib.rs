//! # Mine sweeper
//!
//! A minimalist interface to manage the backend of a Minesweeper game.


mod mine_sweeper;
mod ms_hash;
mod ms_matrix;
mod utils;


pub use mine_sweeper::*;
pub use ms_hash::*;
pub use ms_matrix::*;
pub use utils::*;


#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::{CellContent, MineSweeper};
    use crate::ms_matrix::MSMatrix;
    use crate::utils::iter_neighbors;


    #[test]
    fn it_works() {
        let (h, w, m) = (10, 10, 25);
        let mut ms: MSMatrix = MSMatrix::new(h, w, m).unwrap();
        for i in 0..h {
            for j in 0..1 {
                if let CellContent::Mine = ms.get_cell((i, j)).unwrap().content {
                    ms.toggle_flag((i, j)).ok();
                }
            }
        }
        println!("{}\n", ms);
        for i in 0..h {
            for j in 0..w {
                println!("{:?}", ms.open((i, j)).unwrap());
                println!("{}\n\n", ms);
            }
        }
    }


    #[test]
    #[should_panic]
    fn it_panics() {
        let (h, w) = (10, 10);
        let m = w * h;
        MSMatrix::new(h, w, m).unwrap();
    }


    #[test]
    fn neighbors() {
        let (h, w) = (10, 10);
        let mut neighbors: HashSet<_> = iter_neighbors((0, 0), h, w).unwrap().collect();
        assert_eq!(neighbors, HashSet::from([(1, 1), (0, 1), (1, 0)]));

        neighbors = iter_neighbors((h - 1, w - 1), h, w).unwrap().collect();
        assert_eq!(neighbors, HashSet::from([(h - 2, w - 1), (h - 2, w - 2), (h - 1, w - 2)]));

        neighbors = iter_neighbors((h - 1, w - 2), h, w).unwrap().collect();
        assert_eq!(neighbors, HashSet::from([(h - 1, w - 3), (h - 2, w - 1), (h - 2, w - 3), (h - 2, w - 2), (h - 1, w - 1)]));

        neighbors = iter_neighbors((0, 1), h, w).unwrap().collect();
        assert_eq!(neighbors, HashSet::from([(1, 0), (0, 2), (0, 0), (1, 1), (1, 2)]));

        neighbors = iter_neighbors((1, 1), h, w).unwrap().collect();
        assert_eq!(neighbors, HashSet::from([(1, 2), (1, 0), (0, 2), (0, 0), (2, 0), (2, 1), (2, 2), (0, 1)]));
    }
}
