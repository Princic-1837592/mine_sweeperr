use crate::{Cell, CellContent, CellState, Coordinate, Error, Result, MineSweeper,
            OpenResult, iter_neighbors, random::{Rng, gen_range}};
use std::fmt::{Display, Formatter};
use std::collections::VecDeque;


/// Represents the grid using a matrix of [`cells`](Cell).
/// Use this when you want to load the whole grid in memory at the beginning.
/// Has higher performances when opening cells but takes more memory.
#[derive(Debug, Clone)]
pub struct MSMatrix {
    height: usize,
    width: usize,
    mines: usize,
    cells: Vec<Vec<Cell>>,
}


impl MSMatrix {
    /// Creates a new instance.
    fn new_unchecked(height: usize, width: usize, mines: usize) -> Self {
        Self {
            height,
            width,
            mines,
            cells: vec![vec![Cell::default(); width]; height],
        }
    }

    /// Randomizes the positions of mines when initializing the board.
    fn randomize_mines(&mut self, mines: usize, rng: &mut impl Rng) {
        let mut mines_left = mines;
        while mines_left > 0 {
            let coord @ (x, y) = (gen_range(rng, 0..self.height), gen_range(rng, 0..self.width));
            if let CellContent::Number(_) = self.cells[x][y].content {
                self.cells[x][y].content = CellContent::Mine;
                self.increment_neighbors(coord);
                mines_left -= 1;
            }
        }
    }

    /// Increments the value of all neighboring non-mine cells when initializing the board.
    fn increment_neighbors(&mut self, coord: Coordinate) {
        iter_neighbors(coord, self.height, self.width)
            .unwrap()
            .for_each(|(x, y)| {
                if let CellContent::Number(n) = self.cells[x][y].content {
                    self.cells[x][y].content = CellContent::Number(n + 1);
                }
            });
    }

    /// Counts the number of flags around a cell to propagate the opening procedure.
    fn count_neighboring_flags(&self, coord: Coordinate) -> u8 {
        iter_neighbors(coord, self.height, self.width)
            .unwrap()
            .filter(|(x, y)| self.cells[*x][*y].state == CellState::Flagged)
            .count() as u8
    }

    /// Checks the validity of a coordinate.
    fn check_coordinate(&self, (x, y): Coordinate) -> Result<()> {
        if x < self.height && y < self.width {
            Ok(())
        } else {
            Err(Error::OutOfBounds)
        }
    }
}


impl MineSweeper for MSMatrix {
    fn from_rng(height: usize, width: usize, mines: usize, rng: &mut impl Rng) -> Result<Self> {
        if mines >= height * width {
            return Err(Error::TooManyMines);
        }
        if height == 0 || width == 0 {
            return Err(Error::InvalidParameters);
        }
        let mut result = Self::new_unchecked(height, width, mines);
        result.randomize_mines(mines, rng);
        Ok(result)
    }

    /// Implements all the additional rules suggested in the [trait interface](MineSweeper::open).
    ///
    /// The opening procedure is made using a [queue](VecDeque) (not recursive).
    fn open(&mut self, coord @ (x, y): Coordinate) -> Result<OpenResult> {
        self.check_coordinate(coord)?;
        let (mut cells_opened, mut mines_exploded, mut flags_touched) = (0_usize, 0_usize, 0_usize);
        let mut queue = VecDeque::from([coord]);
        while !queue.is_empty() {
            let coord @ (x, y) = queue.pop_front().unwrap();
            /*match self.cells[x][y].state {
                CellState::Closed => {
                    self.cells[x][y].state = CellState::Open;
                    cells_opened += 1;
                    if self.cells[x][y].content == CellContent::Mine {
                        mines_exploded += 1;
                    }
                    if let CellContent::Number(neighboring_mines) = self.cells[x][y].content {
                        if self.count_neighboring_flags(coord) >= neighboring_mines {
                            iter_neighbors((x, y), self.height, self.width)
                                .unwrap()
                                .filter(|(x, y)| self.cells[*x][*y].state != CellState::Open)
                                .for_each(|coord| queue.push_back(coord));
                        }
                    }
                }
                CellState::Flagged => flags_touched += 1,
                _ => (),
            }*/
            if self.cells[x][y].state == CellState::Flagged {
                flags_touched += 1;
            } else {
                if self.cells[x][y].state == CellState::Closed {
                    self.cells[x][y].state = CellState::Open;
                    cells_opened += 1;
                    if self.cells[x][y].content == CellContent::Mine {
                        mines_exploded += 1;
                    }
                }
                if let CellContent::Number(neighboring_mines) = self.cells[x][y].content {
                    if self.count_neighboring_flags(coord) >= neighboring_mines {
                        iter_neighbors((x, y), self.height, self.width)
                            .unwrap()
                            .filter(|&(x, y)| self.cells[x][y].state != CellState::Open)
                            .for_each(|coord| queue.push_back(coord));
                    }
                }
            }
        }
        Ok(OpenResult::new(self.cells[x][y], cells_opened, mines_exploded, flags_touched))
    }

    fn toggle_flag(&mut self, coord @ (x, y): Coordinate) -> Result<CellState> {
        self.check_coordinate(coord)?;
        match self.cells[x][y].state {
            CellState::Closed => {
                self.cells[x][y].state = CellState::Flagged;
                Ok(CellState::Flagged)
            }
            CellState::Flagged => {
                self.cells[x][y].state = CellState::Closed;
                Ok(CellState::Closed)
            }
            _ => Err(Error::AlreadyOpen),
        }
    }

    fn get_cell(&self, coord @ (x, y): Coordinate) -> Result<Cell> {
        self.check_coordinate(coord)?;
        Ok(self.cells[x][y])
    }

    fn height(&self) -> usize {
        self.height
    }

    fn width(&self) -> usize {
        self.width
    }

    fn mines(&self) -> usize {
        self.mines
    }
}


impl Display for MSMatrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        MineSweeper::fmt(self, f)
    }
}
