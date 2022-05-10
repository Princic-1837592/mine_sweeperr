use std::fmt::Display;


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
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        static NUMBERS: [&str; 9] = ["🟩", "1️⃣", "2️⃣", "3️⃣", "4️⃣", "5️⃣", "6️⃣", "7️⃣", "8️⃣"];
        match self.state {
            CellState::Closed => write!(f, "🟪"),
            CellState::Open => match self.content {
                CellContent::Mine => write!(f, "🟥"),
                CellContent::Number(n) => write!(f, "{}", NUMBERS[n as usize]),
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
    fn new(height: usize, width: usize, mines: usize) -> Result<Self>;
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
    fn get_cell(&self, coord: Coordinate) -> Result<Cell>;
}
