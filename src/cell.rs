use std::fmt::{Display, Formatter};

use crate::NUMBERS;

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

impl Display for CellContent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CellContent::Mine => write!(f, "*"),
            CellContent::Number(n) if *n > 0 => write!(f, "{}", n),
            CellContent::Number(_) => write!(f, " "),
        }
    }
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
