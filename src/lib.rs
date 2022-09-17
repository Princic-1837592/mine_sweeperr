//! # Mine sweeper
//!
//! An easy-to-use interface to manage the backend of a mine sweeper game.
//!
//! Import [`MineSweeper`](MineSweeper) and one of its implementations
//! - [`MSMatrix`](MSMatrix) (recommended)
//! - [`MSHash`](MSHash)
//!
//! to use it.
//! ```
//! use mine_sweeperr::{MSMatrix, MineSweeper, Difficulty, solver::NonDeterministic};
//!
//! // Create a new game with a 16x16 board and 40 mines
//! // setting the starting point at (0, 0)
//! let mut ms: MSMatrix = MSMatrix::new::<NonDeterministic>(Difficulty::medium(), (0, 0)).unwrap();
//!
//! // Reveal the cell at (0, 0)
//! ms.open((0, 0)).unwrap();
//! ```
//! You can also create your own implementation, if you prefer:
//! this crate already declares the needed functions and types to create and manage a mine sweeper game.
//!
//! Read the [CHANGELOG](https://github.com/Princic-1837592/mine_sweeperr/blob/main/CHANGELOG.md) for information about the latest changes.
//!
//! <span style="color:red">**IMPORTANT**</span>:
//! This crate supports [wasm](https://crates.io/crates/wasm-bindgen) but, in that case,
//! seeded random generators (or in general the rand crate) are not allowed
//! due to incompatibility with wasm itself.
//! Maybe in future versions some kind of trick will be implemented to make it work.
//! A [working implementation](https://princic-1837592.github.io/mine_sweeper/index.html) of this library with wasm frontend
//! is available on [my GitHub page](https://Princic-1837592.github.io)

#![allow(unused)]

use std::fmt::{Display, Formatter};

use rand::Rng;

pub use cell::*;
pub use difficulty::*;
pub use implementations::*;
use solver::Solver;
pub use utils::*;

mod implementations;
mod macros;
pub mod solver;
mod utils;

mod cell;
mod difficulty;
#[cfg(test)]
mod tests;

/// A pair of zero-based coordinates. The first coordinate is the row, the second is the column.
pub type Coordinate = (usize, usize);
/// The result of some potentially dangerous action.
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for the [`MineSweeper`](MineSweeper) game.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    OutOfBounds,
    AlreadyOpen,
    TooManyMines,
    InvalidParameters,
}

/// The result of opening a [`cell`](Cell).
///
/// Contains information about the content of the opened cell,
/// how many cells have been opened in total,
/// how many mines have been found (exploded) during the process,
/// the number of flags touched while opening.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OpenResult {
    pub cell: Cell,
    pub cells_opened: usize,
    pub mines_exploded: usize,
    /// The number of times that the opening procedure tried to open a cell near to a flagged cell.
    /// This value may be a lot higher than the number of flagged cells,
    /// since the same flagged cell may be touched multiple times during the opening process.
    pub flags_touched: usize,
}

impl OpenResult {
    pub(crate) fn new(
        cell: Cell,
        cells_opened: usize,
        mines_exploded: usize,
        flags_touched: usize,
    ) -> Self {
        OpenResult {
            cell,
            cells_opened,
            mines_exploded,
            flags_touched,
        }
    }
}

/// Represents the current state of the game
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameState {
    pub flagged: usize,
    pub opened: usize,
    /// This is simply the number of mines minus the number of flagged cells.
    /// This takes into consideration flags regardless of whether they are correct or not.
    pub mines_left: usize,
}

/// Represents a board with its cells.
///
/// Declares methods to create a new instance, to open and flag cells
/// and to access the content and the state of a cell.
///
/// <span style="color:red">**IMPORTANT**</span>:
/// This trait and the included implementations are only the `BACKEND` of the game.
/// No user interaction nor frontend are implemented here, only the functions that you can call
/// from the frontend to interact with the game.
pub trait MineSweeper: Sized {
    /// Creates a new instance of the game given a starting point.
    /// The starting point is granted to be a safe cell and also to be surrounded by safe cells (starting point is a `0`).
    ///
    /// - Returns [`TooManyMines`](Error::TooManyMines) if the number of mines is greater than the number of cells.
    /// The "number of cells" is intended as the total number of cells minus the 9 safe cells granted as starting point.
    /// - Returns [`InvalidParameters`](Error::InvalidParameters) if the number of rows or columns is `0`.
    /// - Returns [`OutOfBounds`](Error::OutOfBounds) if the starting point is out of bounds.
    ///
    /// If not overridden, the default rng used is [`rand::thread_rng()`](rand::thread_rng()).
    fn new<S>(difficulty: Difficulty, start_from: Coordinate) -> Result<Self>
    where
        S: Solver<Self>,
    {
        Self::from_rng::<S, _>(difficulty, start_from, &mut rand::thread_rng())
    }
    /// Creates a new instance of the game using the given random generator.
    /// Can be used to test the game or to reproduce a specific game by passing a seeded rng.
    /// Read more about constraints in a newly created game [here](MineSweeper::new).
    fn from_rng<S, R>(difficulty: Difficulty, start_from: Coordinate, rng: &mut R) -> Result<Self>
    where
        S: Solver<Self>,
        R: Rng;
    /// Tries to open a cell.
    ///
    /// Returns an error if the cell is out of bounds,
    /// otherwise returns an [`OpenResult`](OpenResult).
    ///
    /// The opening procedure should respect the following rules,
    /// that are not enforced by the game but make the user experience better:
    /// - if the opened cell is a number and it's surrounded by enough flags,
    /// all the neighboring non-flagged cells are considered safe to open
    /// and should therefore be opened
    /// - the opening procedure should not stop at the first mine found,
    /// but should keep opening until all safe neighboring cells are opened
    fn open(&mut self, coord: Coordinate) -> Result<OpenResult>;
    /// todo
    fn open_one(&mut self, coord: Coordinate) -> Result<CellContent>;
    /// Tries to toggle the flag on a cell.
    ///
    /// - Returns [`OutOfBounds`](Error::OutOfBounds) if the given coordinate is out of bounds
    /// - Returns [`AlreadyOpen`](Error::AlreadyOpen) if the cell is already open.
    /// - Otherwise returns the new state of the given cell.
    fn toggle_flag(&mut self, coord: Coordinate) -> Result<CellState>;
    /// Returns the state of the given cell.
    fn get_cell(&self, coord: Coordinate) -> Result<Cell>;
    /// Returns the height of the board.
    fn height(&self) -> usize;
    /// Returns the width of the board.
    fn width(&self) -> usize;
    /// Returns the number of mines of the board.
    fn mines(&self) -> usize;
    /// Returns the first cell opened
    fn started_from(&self) -> Coordinate;
    /// Returns the current state of the game
    fn get_game_state(&self) -> GameState;
    /// Displays the grid in a human-readable format as a grid of characters or emojis representing cells.
    ///
    /// - If `#` is given as formatting option, it will be passed to the cells to [format them as emojis](Cell::fmt).
    /// - If the precision parameter `.0` is passed, row and columns numbers will be printed
    /// on the top and left of the grid. No other number is allowed as precision at the moment.
    /// - You can combine `#.0` to print both cells and row-column numbers as emojis.
    ///
    /// The default implementation relies on the implementation of [`get_cell`](MineSweeper::get_cell),
    /// [`height`](MineSweeper::height) and [`width`](MineSweeper::width).
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (use_emojis, print_numbers) = (f.alternate(), f.precision() == Some(0));
        let max_height_digits = (self.height() - 1).to_string().len();
        if print_numbers {
            f.write_str(&get_column_numbers(self.height(), self.width(), use_emojis))?;
        }
        for i in 0..self.height() {
            if print_numbers {
                write!(
                    f,
                    "{}{}",
                    &get_row_number(i, max_height_digits, use_emojis),
                    ROW_NUMBER_RIGHT_SEPARATOR
                )?;
            }
            for j in 0..self.width() {
                self.get_cell((i, j)).unwrap().fmt(f)?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
    // // todo
    // fn from_triple(height: usize, width: usize, mines: Vec<Coordinate>) -> Self;
}
