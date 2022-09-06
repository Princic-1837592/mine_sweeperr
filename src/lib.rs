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
//! use mine_sweeperr::{MSMatrix, MineSweeper, Difficulty};
//!
//! // Create a new game with a 16x16 board and 40 mines
//! // setting the starting point at (0, 0)
//! let mut ms: MSMatrix = MSMatrix::new(Difficulty::medium(), (0, 0)).unwrap();
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

mod implementations;
mod macros;
mod utils;

pub use implementations::*;
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

/// TODO Represents the difficulty of a game in terms of height, width and number of mines.
///
/// When calling [`MineSweeper::new`](MineSweeper::new) or [`MineSweeper::from_rng`](MineSweeper::from_rng)
/// you can either pass a default difficulty or a custom one.
///
/// The default difficulties are:
/// - `Easy`: `9x9` grid with `10` mines
/// - `Medium`: `16x16` grid with `40` mines
/// - `Hard`: `16x30` grid with `99` mines
///
/// Difficulty can be derived from a tuple representing `(height, width, mines)`
/// or from a tuple representing `(height, width, density)`.
/// For example:
/// ```
/// # use mine_sweeperr::Difficulty;
/// let difficulty: Difficulty = (10, 10, 0.1).into();
/// ```
/// will produce a difficulty with `10x10` grid and `10` mines.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Difficulty {
    height: usize,
    width: usize,
    mines: usize,
    deterministic: bool,
}

impl Difficulty {
    const fn new(height: usize, width: usize, mines: usize) -> Self {
        Difficulty {
            height,
            width,
            mines,
            deterministic: true,
        }
    }

    pub const fn easy() -> Self {
        Self::new(9, 9, 10)
    }

    pub const fn medium() -> Self {
        Self::new(16, 16, 40)
    }

    pub const fn hard() -> Self {
        Self::new(16, 30, 99)
    }

    pub const fn custom(height: usize, width: usize, mines: usize) -> Self {
        Self::new(height, width, mines)
    }

    pub fn from_density(height: usize, width: usize, density: f32) -> Self {
        Self::new(height, width, ((height * width) as f32 * density) as usize)
    }

    pub const fn deterministic(self) -> Self {
        Self {
            deterministic: true,
            ..self
        }
    }

    pub const fn non_deterministic(self) -> Self {
        Self {
            deterministic: false,
            ..self
        }
    }
}
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum Difficulty {
//     Easy,
//     Medium,
//     Hard,
//     Custom(usize, usize, usize),
// }

impl From<Difficulty> for (usize, usize, usize) {
    fn from(difficulty: Difficulty) -> (usize, usize, usize) {
        (difficulty.height, difficulty.width, difficulty.mines)
    }
}
impl From<Difficulty> for (usize, usize, usize, bool) {
    fn from(difficulty: Difficulty) -> (usize, usize, usize, bool) {
        (
            difficulty.height,
            difficulty.width,
            difficulty.mines,
            difficulty.deterministic,
        )
    }
}

impl From<(usize, usize, usize)> for Difficulty {
    fn from((height, width, mines): (usize, usize, usize)) -> Difficulty {
        Difficulty::custom(height, width, mines)
    }
}

impl From<(usize, usize, f32)> for Difficulty {
    fn from((height, width, density): (usize, usize, f32)) -> Difficulty {
        Difficulty::from_density(height, width, density)
    }
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
    fn new(difficulty: Difficulty, start_from: Coordinate) -> Result<Self> {
        Self::from_rng(difficulty, start_from, &mut rand::thread_rng())
    }
    /// Creates a new instance of the game using the given random generator.
    /// Can be used to test the game or to reproduce a specific game by passing a seeded rng.
    /// Read more about constraints in a newly created game [here](MineSweeper::new).
    fn from_rng(
        difficulty: Difficulty,
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
mod test_implementations {
    use crate::{iter_neighbors, CellContent, Difficulty, Error, MSHash, MSMatrix, MineSweeper};
    use rand::{rngs::StdRng, Rng, SeedableRng};
    use std::fmt::Display;

    #[test]
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    fn play() {
        fn test<T: MineSweeper + Display>() {
            let mut rng = rand::thread_rng();
            let difficulty = Difficulty::hard();
            let (h, w, m) = difficulty.into();
            let start_from = (rng.gen_range(0..h), rng.gen_range(0..w));
            let mut ms = T::from_rng(difficulty, start_from, &mut rng).unwrap();

            assert_eq!(ms.height(), h);
            assert_eq!(ms.width(), w);
            assert_eq!(ms.mines(), m);

            // flags 60% of the mines
            for i in 0..h {
                for j in 0..w {
                    if let CellContent::Mine = ms.get_cell((i, j)).unwrap().content {
                        if rng.gen_range(0..100) <= 60 {
                            ms.toggle_flag((i, j)).ok();
                        }
                    }
                }
            }
            // println!("{:}\n", ms);

            // opens all cells
            let mut open_result;
            for i in 0..h {
                for j in 0..w {
                    open_result = ms.open((i, j)).unwrap();
                    // println!("{:?}", open_result);
                    // println!("{}\n\n", ms);
                }
            }
            // println!("{:}\n", ms);
        }

        test::<MSMatrix>();
        test::<MSHash>();
    }

    #[test]
    fn invalid_number_of_mines() {
        fn test<T: MineSweeper>() {
            let (h, w) = (10, 15);
            let mut m = w * h;
            let mut difficulty = Difficulty::custom(h, w, m);
            let start_from = (0, 0);

            match T::new(difficulty, start_from) {
                Err(Error::TooManyMines) => (),
                Err(_) => {
                    panic!("Wrong error: MineSweeper::new should panic with Error::TooManyMines!")
                }
                Ok(_) => panic!("MineSweeper::new should panic!"),
            }

            m = w * h - 9;
            difficulty = Difficulty::custom(h, w, m);
            match T::new(difficulty, start_from) {
                Err(Error::TooManyMines) => (),
                Err(_) => {
                    panic!("Wrong error: MineSweeper::new should panic with Error::TooManyMines!")
                }
                Ok(_) => panic!("MineSweeper::new should panic!"),
            }

            m = w * h - 10;
            difficulty = Difficulty::custom(h, w, m);
            assert!(T::new(difficulty, start_from).is_ok());
        }

        test::<MSMatrix>();
        test::<MSHash>();
    }

    #[test]
    fn start_from() {
        fn test<T: MineSweeper>() {
            let mut rng = rand::thread_rng();
            for _ in 0..100 {
                let difficulty = Difficulty::hard();
                let (h, w, _) = difficulty.into();
                let start_from = (rng.gen_range(0..h), rng.gen_range(0..w));
                let ms: T = T::new(difficulty, start_from).unwrap();

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
            let difficulty = Difficulty::hard();
            let (h, w, _) = difficulty.into();
            let start_from = (h, w);

            match T::new(difficulty, start_from) {
                Err(Error::OutOfBounds) => (),
                Err(_) => {
                    panic!("Wrong error: MineSweeper::new should panic with Error::OutOfBounds!")
                }
                Ok(_) => panic!("MineSweeper::new should panic!"),
            }

            let start_from = (h - 1, w - 1);
            assert!(T::new(difficulty, start_from).is_ok());
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
            let seed = rand::thread_rng().gen();
            let mut rng = StdRng::seed_from_u64(seed);
            let difficulty = Difficulty::custom(10, 15, 25);
            let (h, w, m) = difficulty.into();
            let start_from = (rng.gen_range(0..h), rng.gen_range(0..w));
            let mut ms_1 = T::from_rng(difficulty, start_from, &mut rng.clone()).unwrap();
            let mut ms_2 = E::from_rng(difficulty, start_from, &mut rng.clone()).unwrap();

            assert_eq!(ms_1.to_string(), ms_2.to_string());

            // compares the raw content of all cells between the two implementations
            // and flags 5% of the mines, comparing again
            for i in 0..h {
                for j in 0..w {
                    assert_eq!(ms_1.get_cell((i, j)), ms_2.get_cell((i, j)), "ciao");
                    if let CellContent::Mine = ms_1.get_cell((i, j)).unwrap().content {
                        if rng.gen_range(0..100) <= 5 {
                            assert_eq!(ms_1.toggle_flag((i, j)), ms_2.toggle_flag((i, j)));
                        }
                    }
                }
            }
            assert_eq!(format!("{:#}", ms_1), format!("{:#}", ms_2));

            let (mut ms_1_open, mut ms_2_open);
            // opening the whole grid and comparing strings could take a lot of time for big grids
            // or when the grid has a lot of flags
            for i in 0..h {
                for j in 0..w {
                    ms_1_open = ms_1.open((i, j)).unwrap();
                    ms_2_open = ms_2.open((i, j)).unwrap();
                    assert_eq!(ms_1_open, ms_2_open);
                    assert_eq!(format!("{:#}", ms_1), format!("{:#}", ms_2));
                }
            }
        }

        test::<MSMatrix, MSHash>();
    }
}

#[cfg(test)]
mod test_formatter {
    use crate::{MSMatrix, MineSweeper};

    #[test]
    fn simple_formatter() {
        let start_from = (0, 0);
        let mut ms = MSMatrix::new((5, 5, 5).into(), start_from).unwrap();
        let mut expected = r#"
CCCCC
CCCCC
CCCCC
CCCCC
CCCCC
"#[1..]
            .to_string();
        assert_eq!(expected, format!("{:}", ms));

        ms = MSMatrix::new((5, 11, 5).into(), start_from).unwrap();
        expected = r#"
CCCCCCCCCCC
CCCCCCCCCCC
CCCCCCCCCCC
CCCCCCCCCCC
CCCCCCCCCCC
"#[1..]
            .to_string();
        assert_eq!(expected, format!("{:}", ms));

        ms = MSMatrix::new((11, 12, 5).into(), start_from).unwrap();
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
        let mut ms = MSMatrix::new((5, 5, 5).into(), start_from).unwrap();
        let mut expected = r#"
🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪
"#[1..]
            .to_string();
        assert_eq!(expected, format!("{:#}", ms));

        ms = MSMatrix::new((5, 11, 5).into(), start_from).unwrap();
        expected = r#"
🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
"#[1..]
            .to_string();
        assert_eq!(expected, format!("{:#}", ms));

        ms = MSMatrix::new((11, 12, 5).into(), start_from).unwrap();
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
        let mut ms = MSMatrix::new((5, 5, 5).into(), start_from).unwrap();
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

        ms = MSMatrix::new((5, 11, 5).into(), start_from).unwrap();
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

        ms = MSMatrix::new((11, 12, 5).into(), start_from).unwrap();
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
        let mut ms = MSMatrix::new((5, 5, 5).into(), start_from).unwrap();
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

        ms = MSMatrix::new((5, 11, 5).into(), start_from).unwrap();
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

        ms = MSMatrix::new((11, 12, 5).into(), start_from).unwrap();
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

#[cfg(test)]
mod test_types {
    use crate::Difficulty;
    #[test]
    fn difficulty() {
        let mut difficulty: Difficulty;

        difficulty = (10, 10, 0.1).into();
        assert_eq!(difficulty, Difficulty::custom(10, 10, 10));

        difficulty = (10, 10, 1.0).into();
        assert_eq!(difficulty, Difficulty::custom(10, 10, 100));

        difficulty = (10, 10, 0.0).into();
        assert_eq!(difficulty, Difficulty::custom(10, 10, 0));

        difficulty = (10, 10, 0.5).into();
        assert_eq!(difficulty, Difficulty::custom(10, 10, 50));
    }
}
