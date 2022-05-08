use std::fmt::Display;


pub type Coordinate = (usize, usize);
pub type Result<T> = std::result::Result<T, Error>;


/// Error type for the [`minesweeper`](MineSweeper) game.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    OutOfBounds,
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
        Cell {
            state: CellState::Closed,
            content: CellContent::Number(0),
        }
    }
    /// Creates a new cell with state [`open`](CellState::Open) and content [`0`](CellContent::Number).
    pub fn open() -> Self {
        Cell {
            state: CellState::Open,
            content: CellContent::Number(0),
        }
    }
}


impl Default for Cell {
    /// Creates a [`closed`](Cell::closed) cell.
    fn default() -> Self {
        Cell::closed()
    }
}


impl Display for Cell {
    /// Prints a cell as an emoji: 🟪 for closed cells, 🟥 for bomb cells and 🚩 for flagged cells.
    /// Prints a number if the cell is open and contains a positive number, or 🟩 if the number is 0.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        static NUMBERS: [&str; 9] = ["🟩", "1️⃣", "2️⃣", "3️⃣", "4️⃣", "5️⃣", "6️⃣", "7️⃣", "8️⃣"];
        match self.state {
            CellState::Closed => write!(f, "🟪"),
            CellState::Open => match self.content {
                CellContent::Mine => write!(f, "🟥"),
                CellContent::Number(n) => write!(f, "{}", NUMBERS[n as usize]),
            },
            CellState::Flagged => write!(f, "🚩"),
        }
    }
}


/// A board with its cells.
/// Provides methods to create a new instance, to open and flag cells
/// and to access the content and the state of a cell.
pub trait MineSweeper {
    /// Creates a new instance of the game.
    fn new(width: usize, height: usize, mines: usize) -> Self;
    /// Tries to open a cell. Returns an error if the given coordinate is out of bounds.
    /// If the cell is already open or flagged, returns None.
    /// Otherwise returns the content of the opened cell.
    fn open(&mut self, coord: Coordinate) -> Result<Option<CellContent>>;
    ///Tries to flag a cell. Returns an error if the given coordinate is out of bounds.
    /// If the cell is already open or flagged, returns None.
    /// Otherwise returns the content of the flagged cell.
    fn toggle_flag(&mut self, coord: Coordinate) -> Result<Option<CellState>>;
    fn get_cell(&self, coord: Coordinate) -> Result<Option<Cell>>;
}
//⬛🟩🟧🟨🟫⬜🟪🟦🟥
