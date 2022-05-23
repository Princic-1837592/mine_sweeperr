//! # Mine sweeper
//!
//! A minimalist interface to manage the backend of a Minesweeper game.
//!
//! This crate supports [wasm](https://crates.io/crates/wasm-bindgen) but, in that case,
//! seeded random generators are not allowed due to incompatibility.
//! Maybe in future versions some kind of trick will be implemented to make it work.


mod ms_hash;
mod ms_matrix;
mod utils;
mod random;


use std::fmt::Display;
pub use ms_hash::*;
pub use ms_matrix::*;
pub use utils::*;


/// A pair of coordinates. The first coordinate is the row, the second is the column.
pub type Coordinate = (usize, usize);
/// The result of some potentially dangerous action.
pub type Result<T> = std::result::Result<T, Error>;


/// Error type for the [`minesweeper`](MineSweeper) game.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    OutOfBounds,
    AlreadyOpen,
    TooManyMines,
    InvalidParameters,
}


/// The result of opening a [`cell`](Cell).
///
/// Contains information about the content of the first opened cell,
/// how many cells have been opened in total,
/// how many mines have been found during the process,
/// the number of flags touched while opening.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OpenResult {
    pub cell: Cell,
    pub cells_opened: usize,
    pub mines_exploded: usize,
    /// The number of times that the opening procedure tried to open a cell near to a flagged cell.
    /// This value may be a lot higher than the number of flagged cells,
    /// since the same flagged cell may be touched multiple times during the procedure.
    pub flags_touched: usize,
}


impl OpenResult {
    pub(crate) fn new(cell: Cell, cells_opened: usize, mines_exploded: usize, flags_touched: usize) -> Self {
        OpenResult {
            cell,
            cells_opened,
            mines_exploded,
            flags_touched,
        }
    }
}


/// The state of a [`cell`](Cell).
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum CellState {
    Closed,
    Open,
    Flagged,
}


/// The content of a [`cell`](Cell).
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum CellContent {
    Mine,
    Number(u8),
}


/// A cell with its [`state`](CellState) and [`content`](CellContent).
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Cell {
    pub state: CellState,
    pub content: CellContent,
}


impl Cell {
    /// Creates a new cell with the given state and content.
    pub fn new(state: CellState, content: CellContent) -> Self {
        Cell { state, content }
    }
    /// Creates a new cell with state [`closed`](CellState::Closed) and content [`0`](CellContent::Number).
    pub fn closed() -> Self {
        Self::new(CellState::Closed, CellContent::Number(0))
    }
    /// Creates a new cell with state [`open`](CellState::Open) and content [`0`](CellContent::Number).
    pub fn open() -> Self {
        Self::new(CellState::Open, CellContent::Number(0))
    }
}


impl Default for Cell {
    /// Creates a [`closed`](Cell::closed) cell.
    fn default() -> Self {
        Cell::closed()
    }
}


impl Display for Cell {
    /// Prints a cell as an emoji: 🟪 for closed cells, 🟥 for bomb cells and 🟨 for flagged cells.
    /// Prints a number if the cell is open and contains a positive number, or 🟩 if the number is 0.
    // some options are: 🟩 🟨 🟦 🟫 🟧 🟪 🟥
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.state {
            CellState::Closed => write!(f, "🟪"),
            CellState::Open => match self.content {
                CellContent::Mine => write!(f, "🟥"),
                CellContent::Number(n) => write!(f, "{}", if n != 0 { NUMBERS[n as usize] } else { NUMBERS[10] }),
            },
            CellState::Flagged => write!(f, "🟨"),
        }
    }
}


/// A board with its cells.
///
/// Provides methods to create a new instance, to open and flag cells
/// and to access the content and the state of a cell.
pub trait MineSweeper: Sized {
    /// Creates a new instance of the game.
    ///
    /// Returns [`TooManyMines`](Error::TooManyMines) if the number of mines is greater than the number of cells.
    /// Returns [`InvalidParameters`](Error::InvalidParameters) if the number of rows or columns is 0.
    /// If not overridden, the default rng used is [`rand::thread_rng()`](rand::thread_rng()).
    fn new(height: usize, width: usize, mines: usize) -> Result<Self> {
        Self::from_rng(height, width, mines, &mut random::thread_rng())
    }
    /// Creates a new instance of the game using the given random generator.
    /// Can be used to test the game or to reproduce a specific game by passing a seeded rng.
    fn from_rng(height: usize, width: usize, mines: usize, rng: &mut impl random::Rng) -> Result<Self>;
    /// Tries to open a cell.
    ///
    /// Returns an error if the cell is out of bounds,
    /// otherwise returns an [`OpenResult`](OpenResult).
    ///
    /// The opening procedure should respect the following rules,
    /// that are not enforced by the game but make the user experience better:
    /// - if the opened cell is a number and is surrounded by enough flags,
    /// all the neighboring non-flagged cells are considered safe to open
    /// and should therefore be opened
    /// - the opening procedure should not stop at the first mine found,
    /// but should keep opening until all safe neighboring cells are opened
    fn open(&mut self, coord: Coordinate) -> Result<OpenResult>;
    /// Tries to flag a cell.
    ///
    /// Returns [`OutOfBounds`](Error::OutOfBounds) if the given coordinate is out of bounds
    /// and [`AlreadyOpen`](Error::AlreadyOpen) if the cell is already open.
    /// Otherwise returns the new state of the given cell.
    fn toggle_flag(&mut self, coord: Coordinate) -> Result<CellState>;
    /// Returns the state of the given cell.
    fn get_cell(&self, coord: Coordinate) -> Result<Cell>;
}


#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use rand::{Rng, SeedableRng, rngs::StdRng};
    use crate::{CellContent, MineSweeper, MSHash, OpenResult, MSMatrix,
                iter_neighbors, get_column_numbers};


    #[test]
    fn compare_hash_matrix() {
        let mut rng = StdRng::seed_from_u64(6);
        let (h, w, m) = (10, 10, 25);
        let mut msm = MSMatrix::from_rng(h, w, m, &mut rng.clone()).unwrap();
        let mut msh = MSHash::from_rng(h, w, m, &mut rng.clone()).unwrap();
        assert_eq!(msm.to_string(), msh.to_string());
        for i in 0..h {
            for j in 0..w {
                assert_eq!(msm.get_cell((i, j)), msh.get_cell((i, j)));
                if let CellContent::Mine = msm.get_cell((i, j)).unwrap().content {
                    if rng.gen_range(0..100) <= 100 {
                        assert_eq!(msm.toggle_flag((i, j)), msh.toggle_flag((i, j)));
                    }
                }
            }
        }
        assert_eq!(format!("{:#}", msm), format!("{:#}", msh));
        // opening the whole grid and comparing strings could take a lot of time for big grids
        for i in 0..h {
            for j in 0..w {
                assert_eq!(msm.open((i, j)), msh.open((i, j)));
                assert_eq!(format!("{:#}", msm), format!("{:#}", msh));
            }
        }
    }


    #[test]
    // #[ignore]
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    fn play_matrix() {
        let mut rng = rand::thread_rng();
        let (h, w, m) = (10, 10, 25);
        let mut ms: MSMatrix = MSMatrix::new(h, w, m).unwrap();
        for i in 0..h {
            for j in 0..w {
                if let CellContent::Mine = ms.get_cell((i, j)).unwrap().content {
                    if rng.gen_range(0..100) <= 60 {
                        ms.toggle_flag((i, j)).ok();
                    }
                }
            }
        }
        println!("{:#}\n", ms);
        let mut open_result: OpenResult;
        for i in 0..h {
            for j in 0..w {
                open_result = ms.open((i, j)).unwrap();
                // println!("{:?}", open_result);
                // println!("{}\n\n", ms);
            }
        }
    }


    #[test]
    // #[ignore]
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    fn play_hash() {
        let mut rng = rand::thread_rng();
        let (h, w, m) = (10, 10, 25);
        let mut ms: MSHash = MSHash::new(h, w, m).unwrap();
        for i in 0..h {
            for j in 0..w {
                if let CellContent::Mine = ms.get_cell((i, j)).unwrap().content {
                    if rng.gen_range(0..100) <= 10 {
                        ms.toggle_flag((i, j)).ok();
                    }
                }
            }
        }
        println!("{:#}\n", ms);
        let mut open_result: OpenResult;
        for i in 0..h {
            for j in 0..w {
                open_result = ms.open((i, j)).unwrap();
                // println!("{:?}", open_result);
                // println!("{}\n\n", ms);
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


    #[test]
    fn test_column_numbers() {
        let mut expected = r#"
🟫  0️⃣1️⃣2️⃣3️⃣4️⃣5️⃣6️⃣7️⃣8️⃣

"#[1..].to_string();
        assert_eq!(expected, get_column_numbers(9, 9));

        expected = r#"
🟫  0️⃣1️⃣2️⃣3️⃣4️⃣5️⃣6️⃣7️⃣8️⃣9️⃣

"#[1..].to_string();
        assert_eq!(expected, get_column_numbers(10, 10));

        expected = r#"
🟫🟫  🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫1️⃣1️⃣1️⃣1️⃣1️⃣
🟫🟫  0️⃣1️⃣2️⃣3️⃣4️⃣5️⃣6️⃣7️⃣8️⃣9️⃣0️⃣1️⃣2️⃣3️⃣4️⃣

"#[1..].to_string();
        assert_eq!(expected, get_column_numbers(15, 15));

        expected = r#"
🟫🟫🟫🟫  🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫1️⃣1️⃣1️⃣1️⃣1️⃣1️⃣1️⃣1️⃣1️⃣1️⃣2️⃣2️⃣2️⃣2️⃣2️⃣
🟫🟫🟫🟫  0️⃣1️⃣2️⃣3️⃣4️⃣5️⃣6️⃣7️⃣8️⃣9️⃣0️⃣1️⃣2️⃣3️⃣4️⃣5️⃣6️⃣7️⃣8️⃣9️⃣0️⃣1️⃣2️⃣3️⃣4️⃣

"#[1..].to_string();
        assert_eq!(expected, get_column_numbers(1250, 25));

        expected = r#"
🟫🟫  🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫1️⃣1️⃣1️⃣1️⃣1️⃣
🟫🟫  🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫1️⃣1️⃣1️⃣1️⃣1️⃣1️⃣1️⃣1️⃣1️⃣1️⃣2️⃣2️⃣2️⃣2️⃣2️⃣2️⃣2️⃣2️⃣2️⃣2️⃣3️⃣3️⃣3️⃣3️⃣3️⃣3️⃣3️⃣3️⃣3️⃣3️⃣4️⃣4️⃣4️⃣4️⃣4️⃣4️⃣4️⃣4️⃣4️⃣4️⃣5️⃣5️⃣5️⃣5️⃣5️⃣5️⃣5️⃣5️⃣5️⃣5️⃣6️⃣6️⃣6️⃣6️⃣6️⃣6️⃣6️⃣6️⃣6️⃣6️⃣7️⃣7️⃣7️⃣7️⃣7️⃣7️⃣7️⃣7️⃣7️⃣7️⃣8️⃣8️⃣8️⃣8️⃣8️⃣8️⃣8️⃣8️⃣8️⃣8️⃣9️⃣9️⃣9️⃣9️⃣9️⃣9️⃣9️⃣9️⃣9️⃣9️⃣0️⃣0️⃣0️⃣0️⃣0️⃣
🟫🟫  0️⃣1️⃣2️⃣3️⃣4️⃣5️⃣6️⃣7️⃣8️⃣9️⃣0️⃣1️⃣2️⃣3️⃣4️⃣5️⃣6️⃣7️⃣8️⃣9️⃣0️⃣1️⃣2️⃣3️⃣4️⃣5️⃣6️⃣7️⃣8️⃣9️⃣0️⃣1️⃣2️⃣3️⃣4️⃣5️⃣6️⃣7️⃣8️⃣9️⃣0️⃣1️⃣2️⃣3️⃣4️⃣5️⃣6️⃣7️⃣8️⃣9️⃣0️⃣1️⃣2️⃣3️⃣4️⃣5️⃣6️⃣7️⃣8️⃣9️⃣0️⃣1️⃣2️⃣3️⃣4️⃣5️⃣6️⃣7️⃣8️⃣9️⃣0️⃣1️⃣2️⃣3️⃣4️⃣5️⃣6️⃣7️⃣8️⃣9️⃣0️⃣1️⃣2️⃣3️⃣4️⃣5️⃣6️⃣7️⃣8️⃣9️⃣0️⃣1️⃣2️⃣3️⃣4️⃣5️⃣6️⃣7️⃣8️⃣9️⃣0️⃣1️⃣2️⃣3️⃣4️⃣

"#[1..].to_string();
        assert_eq!(expected, get_column_numbers(11, 105));
    }
}
