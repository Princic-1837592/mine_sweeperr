use std::collections::{HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use rand::Rng;
use crate::{Cell, CellContent, CellState, Error, iter_neighbors, MineSweeper, OpenResult, Result, Coordinate};


/// **TODO: NOT IMPLEMENTED YET**
///
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
    /// Randomizes the positions of mines when initializing the board.
    fn randomize_mines(&mut self, mines: usize) {
        let mut rng = rand::thread_rng();
        while self.mines.len() < mines {
            let coord = (rng.gen_range(0..self.height), rng.gen_range(0..self.width));
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
    fn new(height: usize, width: usize, mines: usize) -> Result<Self> {
        if mines >= height * width {
            return Err(Error::TooManyMines);
        }
        if width == 0 || height == 0 {
            return Err(Error::InvalidParameters);
        }
        let mut result = MSHash {
            width,
            height,
            open: Default::default(),
            flagged: Default::default(),
            mines: HashSet::with_capacity(mines),
        };
        result.randomize_mines(mines);
        Ok(result)
    }

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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f, "{}",
            (0..self.height).map(|i| (0..self.width)
                .map(move |j| self
                    .get_cell((i, j))
                    .unwrap()
                    .to_string())
                .collect::<Vec<_>>().join(""))
                            .collect::<Vec<String>>().join("\n")
        )
    }
}
