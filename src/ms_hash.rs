use std::collections::{HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use crate::{Cell, CellContent, CellState, Error, iter_neighbors, MineSweeper, OpenResult, Result,
            Coordinate, get_column_numbers, get_row_number, ROW_NUMBER_RIGHT_SEPARATOR,
            random::{Rng, gen_range}};


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


impl MSHash {
    /// Creates a new instance.
    fn new_unchecked(width: usize, height: usize, mines: usize) -> Self {
        Self {
            width,
            height,
            open: Default::default(),
            flagged: Default::default(),
            mines: HashSet::with_capacity(mines),
        }
    }

    /// Randomizes the positions of mines when initializing the board.
    fn randomize_mines(&mut self, mines: usize, rng: &mut impl Rng) {
        while self.mines.len() < mines {
            let coord = (gen_range(rng, 0, self.height), gen_range(rng, 0, self.width));
            if !self.mines.contains(&coord) {
                self.mines.insert(coord);
            }
        }
    }

    /// Checks the validity of a coordinate.
    fn check_coordinate(&self, (x, y): Coordinate) -> Result<()> {
        if x < self.height && y < self.width {
            Ok(())
        } else {
            Err(Error::OutOfBounds)
        }
    }

    /// Counts the number of mines around a cell.
    fn count_neighboring_mines(&self, coord: Coordinate) -> u8 {
        iter_neighbors(coord, self.height, self.width)
            .unwrap()
            .filter(|coord| self.mines.contains(coord))
            .count() as u8
    }

    /// Counts the number of flags around a cell to propagate the opening procedure.
    fn count_neighboring_flags(&self, coord: Coordinate) -> u8 {
        iter_neighbors(coord, self.height, self.width)
            .unwrap()
            .filter(|coord| self.flagged.contains(coord))
            .count() as u8
    }
}


impl MineSweeper for MSHash {
    #[cfg(not(target_family = "wasm"))]
    fn from_rng(height: usize, width: usize, mines: usize, rng: &mut impl Rng) -> Result<Self> {
        if mines >= height * width {
            return Err(Error::TooManyMines);
        }
        if width == 0 || height == 0 {
            return Err(Error::InvalidParameters);
        }
        let mut result = Self::new_unchecked(height, width, mines);
        result.randomize_mines(mines, rng);
        Ok(result)
    }

    /// Implements all the additional rules suggested in the [`trait interface`](MineSweeper::open).
    ///
    /// The opening procedure is made using a [`queue`](VecDeque) (not recursive).
    fn open(&mut self, coord: Coordinate) -> Result<OpenResult> {
        self.check_coordinate(coord)?;
        let (mut cells_opened, mut mines_exploded, mut flags_touched) = (0_usize, 0_usize, 0_usize);
        let mut queue = VecDeque::from([coord]);
        let mut cell: Cell;
        while !queue.is_empty() {
            let coord @ (x, y) = queue.pop_front().unwrap();
            cell = self.get_cell(coord).unwrap();
            match cell.state {
                CellState::Closed => {
                    self.open.insert(coord);
                    cells_opened += 1;
                    if cell.content == CellContent::Mine {
                        mines_exploded += 1;
                    }
                    if let CellContent::Number(neighboring_mines) = cell.content {
                        if self.count_neighboring_flags(coord) >= neighboring_mines {
                            iter_neighbors((x, y), self.height, self.width)
                                .unwrap()
                                .filter(|&coord| self.get_cell(coord).unwrap().state != CellState::Open)
                                .for_each(|coord| queue.push_back(coord));
                        }
                    }
                }
                CellState::Flagged => flags_touched += 1,
                _ => (),
            }
        }
        Ok(OpenResult::new(self.get_cell(coord).unwrap(), cells_opened, mines_exploded, flags_touched))
    }

    fn toggle_flag(&mut self, coord: Coordinate) -> Result<CellState> {
        self.check_coordinate(coord)?;
        if self.open.contains(&coord) {
            return Err(Error::AlreadyOpen);
        }
        if self.flagged.contains(&coord) {
            self.flagged.remove(&coord);
            Ok(CellState::Flagged)
        } else {
            self.flagged.insert(coord);
            Ok(CellState::Flagged)
        }
    }

    fn get_cell(&self, coord: (usize, usize)) -> Result<Cell> {
        self.check_coordinate(coord)?;
        let (mut state, mut content) = (CellState::Closed, CellContent::Mine);
        if !self.mines.contains(&coord) {
            content = CellContent::Number(self.count_neighboring_mines(coord));
        }
        if self.open.contains(&coord) {
            state = CellState::Open;
        } else if self.flagged.contains(&coord) {
            state = CellState::Flagged;
        }
        Ok(Cell { state, content })
    }
}


impl Display for MSHash {
    /// Displays the grid in a human-readable format as a grid of emojis representing cells.
    ///
    /// Can be formatted passing the `#` option:
    /// in that case, row numbers will be shown on the left and column numbers on the top.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let print_numbers = f.alternate();
        let max_height_digits = (self.height - 1).to_string().len();
        write!(
            f,
            "{}{}",
            if print_numbers { get_column_numbers(self.height, self.width) } else { String::from("") },
            (0..self.height)
                .map(|i| (0..self.width)
                    .map(|j| self
                        .get_cell((i, j))
                        .unwrap()
                        .to_string())
                    .collect::<String>())
                .enumerate()
                .map(|(i, s)| format!(
                    "{}{}",
                    if print_numbers {
                        format!(
                            "{}{}",
                            get_row_number(i, max_height_digits),
                            ROW_NUMBER_RIGHT_SEPARATOR
                        )
                    } else {
                        String::from("")
                    },
                    s
                ))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
