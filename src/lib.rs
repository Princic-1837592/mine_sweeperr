//! # Mine sweeper
//!
//! A minimalist interface to manage the backend of a mine sweeper game.
//!
//! Import [`MineSweeper`](MineSweeper) and one of its implementations
//! ([`MSMatrix`](MSMatrix) or [`MSHash`](MSHash)) to use it.
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
//! A working implementation of this library with wasm frontend
//! is available on [my GitHub page](https://Princic-1837592.github.io)

mod ms_hash;
mod ms_matrix;
mod utils;

pub use ms_hash::*;
pub use ms_matrix::*;
use std::fmt::{Display, Formatter};
pub use utils::*;

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
pub struct GameState {
    pub flagged: usize,
    pub opened: usize,
    /// This is simply the number of mines minus the number of flagged cells.
    /// This takes into consideration flags regardless of whether they are correct or not.
    pub mines_left: usize,
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
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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
    /// Prints a cell in a human-readable way.
    ///
    /// If no formatting option is given, the following chars are used:
    /// - `C` for closed cells
    /// - `M` for mine cells
    /// - `F` for flagged cells
    /// - ` ` (blank space) for cells with a 0
    /// - `1` to `9` for cells with a number
    ///
    /// If `#` is given as formatting option, the following chars are used:
    /// - `🟪` for closed cells
    /// - `🟥` for mine cells
    /// - `🟨` for flagged cells
    /// - `🟩` for cells with a 0
    /// - `1️⃣` to `9️⃣` for cells with a number
    ///
    /// Other options are ignored.
    ///
    /// <span style="color:red">**IMPORTANT**</span>:
    /// Emojis used in this function are chosen because they have the same width on my machine,
    /// making the grid aligned on columns.
    /// I know that this may not be the same for everyone so i suggest you try and implement
    /// your own formatting function to use the set of characters you think is best for you.
    /// When using monospace fonts, the non-emoji chars are perfectly aligned on columns
    /// but of course they are not the best way to print the grid.
    // some options are: 🟩 🟨 🟦 🟫 🟧 🟪 🟥
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        if f.alternate() {
            match self.state {
                CellState::Closed => write!(f, "🟪"),
                CellState::Open => match self.content {
                    CellContent::Mine => write!(f, "🟥"),
                    CellContent::Number(n) => write!(
                        f,
                        "{}",
                        if n > 0 {
                            NUMBERS[n as usize]
                        } else {
                            NUMBERS[10]
                        }
                    ),
                },
                CellState::Flagged => write!(f, "🟨"),
            }
        } else {
            match self.state {
                CellState::Closed => write!(f, "C"),
                CellState::Open => match self.content {
                    CellContent::Mine => write!(f, "M"),
                    CellContent::Number(n) => {
                        if n > 0 {
                            write!(f, "{}", n)
                        } else {
                            write!(f, " ")
                        }
                    }
                },
                CellState::Flagged => write!(f, "F"),
            }
        }
    }
}

#[macro_export]
macro_rules! check {
    ($mines:ident $height:ident $width:ident $start_from:ident) => {
        if $mines >= $height * $width - 9 {
            return Err(Error::TooManyMines);
        }
        if $height == 0 || $width == 0 {
            return Err(Error::InvalidParameters);
        }
        if $start_from.0 >= $height || $start_from.1 >= $width {
            return Err(Error::OutOfBounds);
        }
    };
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
    fn new(height: usize, width: usize, mines: usize, start_from: Coordinate) -> Result<Self> {
        Self::from_rng(height, width, mines, start_from, &mut rand::thread_rng())
    }
    /// Creates a new instance of the game using the given random generator.
    /// Can be used to test the game or to reproduce a specific game by passing a seeded rng.
    /// Read more about constraints in a newly created game [here](MineSweeper::new).
    fn from_rng(
        height: usize,
        width: usize,
        mines: usize,
        start_from: Coordinate,
        rng: &mut impl rand::Rng,
    ) -> Result<Self>;
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
}

#[cfg(test)]
mod implementations_tests {
    use crate::{iter_neighbors, CellContent, Error, MSHash, MSMatrix, MineSweeper, OpenResult};
    use rand::{rngs::StdRng, Rng, SeedableRng};
    use std::fmt::Display;

    #[test]
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    fn play() {
        fn test<T: MineSweeper>() {
            let mut rng = rand::thread_rng();
            let (h, w, m) = (10, 15, 25);
            let start_from = (rng.gen_range(0..h), rng.gen_range(0..w));
            let mut ms = T::from_rng(h, w, m, start_from, &mut rng).unwrap();
            assert_eq!(ms.height(), h);
            assert_eq!(ms.width(), w);
            assert_eq!(ms.mines(), m);
            for i in 0..h {
                for j in 0..w {
                    if let CellContent::Mine = ms.get_cell((i, j)).unwrap().content {
                        if rng.gen_range(0..100) <= 60 {
                            ms.toggle_flag((i, j)).ok();
                        }
                    }
                }
            }
            // println!("{:#}\n", ms);
            let mut open_result;
            for i in 0..h {
                for j in 0..w {
                    open_result = ms.open((i, j)).unwrap();
                    // println!("{:?}", open_result);
                    // println!("{}\n\n", ms);
                }
            }
            // println!("{:#}\n", ms);
        }
        test::<MSMatrix>();
        test::<MSHash>();
    }

    #[test]
    fn invalid_number_of_mines() {
        fn test<T: MineSweeper>() {
            let (h, w) = (10, 15);
            let m = w * h;
            match T::new(h, w, m, (0, 0)) {
                Err(Error::TooManyMines) => (),
                Err(_) => panic!("Wrong error: MSHash::new should panic as Error::TooManyMines!"),
                Ok(_) => panic!("MSHash::new should panic!"),
            }
            let m = w * h - 10;
            assert!(T::new(h, w, m, (0, 0)).is_ok());
        }
        test::<MSMatrix>();
        test::<MSHash>();
    }

    #[test]
    fn start_from() {
        fn test<T: MineSweeper>() {
            for _ in 0..1000 {
                let mut rng = rand::thread_rng();
                let (h, w, m) = (100, 150, 250);
                let start_from = (rng.gen_range(0..h), rng.gen_range(0..w));
                let ms: T = T::new(h, w, m, start_from).unwrap();
                let mut should_be_safe = iter_neighbors(start_from, h, w)
                    .unwrap()
                    .map(|(r, c)| ms.get_cell((r, c)).unwrap().content);
                assert_eq!(
                    ms.get_cell(start_from).unwrap().content,
                    CellContent::Number(0)
                );
                assert!(!should_be_safe.any(|cell_content| cell_content == CellContent::Mine));
            }
        }
        test::<MSMatrix>();
        test::<MSHash>();
    }

    #[test]
    fn invalid_start_from() {
        fn test<T: MineSweeper>() {
            let (h, w, m) = (10, 15, 25);
            let start_from = (h, w);
            match T::new(h, w, m, start_from) {
                Err(Error::OutOfBounds) => (),
                Err(_) => panic!("Wrong error: MSMatrix::new should panic as Error::OutOfBounds!"),
                Ok(_) => panic!("MSMatrix::new should panic!"),
            }
            let start_from = (h - 1, w - 1);
            assert!(T::new(h, w, m, start_from).is_ok());
        }
        test::<MSMatrix>();
        test::<MSHash>();
    }

    #[test]
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    fn compare_implementations() {
        fn test<T, E>()
        where
            T: MineSweeper + Display,
            E: MineSweeper + Display,
        {
            let mut rng = StdRng::seed_from_u64(6);
            let (h, w, m) = (10, 15, 25);
            let start_from = (rng.gen_range(0..h), rng.gen_range(0..w));
            let mut ms_1 = T::from_rng(h, w, m, start_from, &mut rng.clone()).unwrap();
            let mut ms_2 = E::from_rng(h, w, m, start_from, &mut rng.clone()).unwrap();
            assert_eq!(ms_1.to_string(), ms_2.to_string());
            for i in 0..h {
                for j in 0..w {
                    assert_eq!(ms_1.get_cell((i, j)), ms_2.get_cell((i, j)));
                    if let CellContent::Mine = ms_1.get_cell((i, j)).unwrap().content {
                        if rng.gen_range(0..100) <= 5 {
                            assert_eq!(ms_1.toggle_flag((i, j)), ms_2.toggle_flag((i, j)));
                        }
                    }
                }
            }
            assert_eq!(format!("{:#}", ms_1), format!("{:#}", ms_2));
            let (mut msm_open, mut msh_open): (OpenResult, OpenResult);
            // opening the whole grid and comparing strings could take a lot of time for big grids
            // or when the grid has a lot of flags
            for i in 0..h {
                for j in 0..w {
                    msm_open = ms_1.open((i, j)).unwrap();
                    msh_open = ms_2.open((i, j)).unwrap();
                    assert_eq!(msm_open, msh_open);
                    assert_eq!(format!("{:#}", ms_1), format!("{:#}", ms_2));
                }
            }
        }
        test::<MSMatrix, MSHash>();
    }
}

#[cfg(test)]
mod formatter_tests {
    use crate::{MSMatrix, MineSweeper};

    #[test]
    fn simple_formatter() {
        let start_from = (0, 0);
        let mut ms = MSMatrix::new(5, 5, 5, start_from).unwrap();
        let mut expected = r#"
CCCCC
CCCCC
CCCCC
CCCCC
CCCCC
"#[1..]
            .to_string();
        assert_eq!(expected, format!("{:}", ms));

        ms = MSMatrix::new(5, 11, 5, start_from).unwrap();
        expected = r#"
CCCCCCCCCCC
CCCCCCCCCCC
CCCCCCCCCCC
CCCCCCCCCCC
CCCCCCCCCCC
"#[1..]
            .to_string();
        assert_eq!(expected, format!("{:}", ms));

        ms = MSMatrix::new(11, 12, 5, start_from).unwrap();
        expected = r#"
CCCCCCCCCCCC
CCCCCCCCCCCC
CCCCCCCCCCCC
CCCCCCCCCCCC
CCCCCCCCCCCC
CCCCCCCCCCCC
CCCCCCCCCCCC
CCCCCCCCCCCC
CCCCCCCCCCCC
CCCCCCCCCCCC
CCCCCCCCCCCC
"#[1..]
            .to_string();
        assert_eq!(expected, format!("{:}", ms));
    }

    #[test]
    fn alternate_formatter() {
        let start_from = (0, 0);
        let mut ms = MSMatrix::new(5, 5, 5, start_from).unwrap();
        let mut expected = r#"
🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪
"#[1..]
            .to_string();
        assert_eq!(expected, format!("{:#}", ms));

        ms = MSMatrix::new(5, 11, 5, start_from).unwrap();
        expected = r#"
🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
"#[1..]
            .to_string();
        assert_eq!(expected, format!("{:#}", ms));

        ms = MSMatrix::new(11, 12, 5, start_from).unwrap();
        expected = r#"
🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
"#[1..]
            .to_string();
        assert_eq!(expected, format!("{:#}", ms));
    }

    #[test]
    fn precision_formatter() {
        let start_from = (0, 0);
        let mut ms = MSMatrix::new(5, 5, 5, start_from).unwrap();
        let mut expected = r#"
   01234

0  CCCCC
1  CCCCC
2  CCCCC
3  CCCCC
4  CCCCC
"#[1..]
            .to_string();
        assert_eq!(expected, format!("{:.0}", ms));

        ms = MSMatrix::new(5, 11, 5, start_from).unwrap();
        expected = r#"
             1
   01234567890

0  CCCCCCCCCCC
1  CCCCCCCCCCC
2  CCCCCCCCCCC
3  CCCCCCCCCCC
4  CCCCCCCCCCC
"#[1..]
            .to_string();
        assert_eq!(expected, format!("{:.0}", ms));

        ms = MSMatrix::new(11, 12, 5, start_from).unwrap();
        expected = r#"
              11
    012345678901

 0  CCCCCCCCCCCC
 1  CCCCCCCCCCCC
 2  CCCCCCCCCCCC
 3  CCCCCCCCCCCC
 4  CCCCCCCCCCCC
 5  CCCCCCCCCCCC
 6  CCCCCCCCCCCC
 7  CCCCCCCCCCCC
 8  CCCCCCCCCCCC
 9  CCCCCCCCCCCC
10  CCCCCCCCCCCC
"#[1..]
            .to_string();
        assert_eq!(expected, format!("{:.0}", ms));
    }

    #[test]
    fn full_formatter() {
        let start_from = (0, 0);
        let mut ms = MSMatrix::new(5, 5, 5, start_from).unwrap();
        let mut expected = r#"
🟫  0️⃣1️⃣2️⃣3️⃣4️⃣

0️⃣  🟪🟪🟪🟪🟪
1️⃣  🟪🟪🟪🟪🟪
2️⃣  🟪🟪🟪🟪🟪
3️⃣  🟪🟪🟪🟪🟪
4️⃣  🟪🟪🟪🟪🟪
"#[1..]
            .to_string();
        assert_eq!(expected, format!("{:#.0}", ms));

        ms = MSMatrix::new(5, 11, 5, start_from).unwrap();
        expected = r#"
🟫  🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫1️⃣
🟫  0️⃣1️⃣2️⃣3️⃣4️⃣5️⃣6️⃣7️⃣8️⃣9️⃣0️⃣

0️⃣  🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
1️⃣  🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
2️⃣  🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
3️⃣  🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
4️⃣  🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
"#[1..]
            .to_string();
        assert_eq!(expected, format!("{:#.0}", ms));

        ms = MSMatrix::new(11, 12, 5, start_from).unwrap();
        expected = r#"
🟫🟫  🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫1️⃣1️⃣
🟫🟫  0️⃣1️⃣2️⃣3️⃣4️⃣5️⃣6️⃣7️⃣8️⃣9️⃣0️⃣1️⃣

🟫0️⃣  🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟫1️⃣  🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟫2️⃣  🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟫3️⃣  🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟫4️⃣  🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟫5️⃣  🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟫6️⃣  🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟫7️⃣  🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟫8️⃣  🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟫9️⃣  🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
1️⃣0️⃣  🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
"#[1..]
            .to_string();
        assert_eq!(expected, format!("{:#.0}", ms));
    }
}
