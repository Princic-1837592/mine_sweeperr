use crate::{
    check, iter_neighbors, Cell, CellContent, CellState, Coordinate, Difficulty, Error, GameState,
    MineSweeper, OpenResult, Result,
};
use rand::Rng;
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};

/// Represents the grid using a matrix of [`cells`](Cell).
/// Use this when you want to load the whole grid in memory at the beginning.
/// Has higher performances when opening cells but takes more memory.
#[derive(Debug, Clone)]
pub struct MSMatrix {
    height: usize,
    width: usize,
    mines: usize,
    cells: Vec<Vec<Cell>>,
    start_from: Coordinate,
}

impl MSMatrix {
    /// Creates a new instance.
    fn new_unchecked(height: usize, width: usize, mines: usize, start_from: Coordinate) -> Self {
        Self {
            height,
            width,
            mines,
            cells: vec![vec![Cell::default(); width]; height],
            start_from,
        }
    }

    /// Randomizes the positions of mines when initializing the board.
    fn randomize_mines(&mut self, mines: usize, start_from: Coordinate, rng: &mut impl Rng) {
        let mut mines_left = mines;
        let mut must_be_safe = iter_neighbors(start_from, self.height, self.width)
            .unwrap()
            .collect::<Vec<_>>();
        must_be_safe.push(start_from);
        while mines_left > 0 {
            let coord @ (r, c) = (rng.gen_range(0..self.height), rng.gen_range(0..self.width));
            if let CellContent::Number(_) = self.cells[r][c].content {
                if !must_be_safe.contains(&coord) {
                    self.cells[r][c].content = CellContent::Mine;
                    self.increment_neighbors(coord);
                    mines_left -= 1;
                }
            }
        }
    }

    /// Increments the value of all neighboring non-mine cells when initializing the board.
    fn increment_neighbors(&mut self, coord: Coordinate) {
        iter_neighbors(coord, self.height, self.width)
            .unwrap()
            .for_each(|(r, c)| {
                if let CellContent::Number(n) = self.cells[r][c].content {
                    self.cells[r][c].content = CellContent::Number(n + 1);
                }
            });
    }

    /// Counts the number of flags around a cell to propagate the opening procedure.
    fn count_neighboring_flags(&self, coord: Coordinate) -> u8 {
        iter_neighbors(coord, self.height, self.width)
            .unwrap()
            .filter(|(r, c)| self.cells[*r][*c].state == CellState::Flagged)
            .count() as u8
    }

    /// Checks the validity of a coordinate.
    fn check_coordinate(&self, (r, c): Coordinate) -> Result<()> {
        if r < self.height && c < self.width {
            Ok(())
        } else {
            Err(Error::OutOfBounds)
        }
    }
}

impl MineSweeper for MSMatrix {
    fn from_rng(
        difficulty: Difficulty,
        start_from: Coordinate,
        rng: &mut impl Rng,
    ) -> Result<Self> {
        let difficulty @ (height, width, mines) = difficulty.into();
        check!(difficulty, start_from);
        let mut result = Self::new_unchecked(height, width, mines, start_from);
        result.randomize_mines(mines, start_from, rng);
        Ok(result)
    }

    /// Implements all the additional rules suggested in the [trait interface](MineSweeper::open).
    ///
    /// The opening procedure is made using a [queue](VecDeque) (not recursive).
    fn open(&mut self, coord @ (r, c): Coordinate) -> Result<OpenResult> {
        self.check_coordinate(coord)?;
        let (mut cells_opened, mut mines_exploded, mut flags_touched) = (0_usize, 0_usize, 0_usize);
        let mut queue = VecDeque::from([coord]);
        while !queue.is_empty() {
            let coord @ (r, c) = queue.pop_front().unwrap();
            if self.cells[r][c].state == CellState::Flagged {
                flags_touched += 1;
            } else {
                if self.cells[r][c].state == CellState::Closed {
                    self.cells[r][c].state = CellState::Open;
                    cells_opened += 1;
                    if self.cells[r][c].content == CellContent::Mine {
                        mines_exploded += 1;
                    }
                }
                if let CellContent::Number(neighboring_mines) = self.cells[r][c].content {
                    if neighboring_mines > 0
                        && self.count_neighboring_flags(coord) >= neighboring_mines
                    {
                        queue.extend(
                            iter_neighbors((r, c), self.height, self.width)
                                .unwrap()
                                .filter(|&(r, c)| self.cells[r][c].state != CellState::Open),
                        );
                    }
                }
            }
        }
        Ok(OpenResult::new(
            self.cells[r][c],
            cells_opened,
            mines_exploded,
            flags_touched,
        ))
    }

    fn toggle_flag(&mut self, coord @ (r, c): Coordinate) -> Result<CellState> {
        self.check_coordinate(coord)?;
        match self.cells[r][c].state {
            CellState::Closed => {
                self.cells[r][c].state = CellState::Flagged;
                Ok(CellState::Flagged)
            }
            CellState::Flagged => {
                self.cells[r][c].state = CellState::Closed;
                Ok(CellState::Closed)
            }
            _ => Err(Error::AlreadyOpen),
        }
    }

    fn get_cell(&self, coord @ (r, c): Coordinate) -> Result<Cell> {
        self.check_coordinate(coord)?;
        Ok(self.cells[r][c])
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

    fn started_from(&self) -> Coordinate {
        self.start_from
    }

    fn get_game_state(&self) -> GameState {
        todo!()
    }
}

impl Display for MSMatrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        MineSweeper::fmt(self, f)
    }
}