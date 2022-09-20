use std::collections::{HashSet, VecDeque};
use std::fmt::{Display, Formatter};

use rand::Rng;

use crate::{
    check, count_neighboring_flags, iter_neighbors, solver::Solver, Cell, CellContent, CellState,
    Coordinate, Difficulty, Error, GameState, MineSweeper, OpenResult, Result,
};

/// Represents a grid using [`HashSets`](HashSet) of [`Coordinates`](Coordinate).
/// Use this when you don't want to load the whole grid in memory at the beginning.
/// Has lower performances when opening cells but takes less memory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MSHash {
    height: usize,
    width: usize,
    open: HashSet<Coordinate>,
    flagged: HashSet<Coordinate>,
    mines: HashSet<Coordinate>,
    start_from: Coordinate,
    exploded: usize,
}

impl MSHash {
    /// Creates a new instance.
    fn new_unchecked(height: usize, width: usize, mines: usize, start_from: Coordinate) -> Self {
        Self {
            height,
            width,
            open: Default::default(),
            flagged: Default::default(),
            mines: HashSet::with_capacity(mines),
            start_from,
            exploded: 0,
        }
    }

    /// Randomizes the positions of mines when initializing the board.
    fn randomize_mines(&mut self, mines: usize, start_from: Coordinate, rng: &mut impl Rng) {
        let mut must_be_safe = iter_neighbors(start_from, self.height, self.width)
            .unwrap()
            .collect::<Vec<_>>();
        must_be_safe.push(start_from);
        while self.mines.len() < mines {
            let coord = (rng.gen_range(0..self.height), rng.gen_range(0..self.width));
            if !self.mines.contains(&coord) && !must_be_safe.contains(&coord) {
                self.mines.insert(coord);
            }
        }
    }

    /// Checks the validity of a coordinate.
    fn check_coordinate(&self, (r, c): Coordinate) -> Result<()> {
        if r < self.height && c < self.width {
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

    // /// Counts the number of flags around a cell to propagate the opening procedure.
    // fn count_neighboring_flags(&self, coord: Coordinate) -> u8 {
    //     iter_neighbors(coord, self.height, self.width)
    //         .unwrap()
    //         .filter(|coord| self.flagged.contains(coord))
    //         .count() as u8
    // }
}

impl MineSweeper for MSHash {
    fn from_rng<S: Solver>(
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
    fn open(&mut self, coord: Coordinate) -> Result<OpenResult> {
        self.check_coordinate(coord)?;
        let (mut cells_opened, mut mines_exploded, mut flags_touched) = (0, 0, 0);
        let mut queue = VecDeque::from([coord]);
        let mut cell: Cell;
        while !queue.is_empty() {
            let coord = queue.pop_front().unwrap();
            cell = self.get_cell(coord).unwrap();
            if cell.state == CellState::Flagged {
                flags_touched += 1;
            } else {
                if cell.state == CellState::Closed {
                    self.open.insert(coord);
                    cells_opened += 1;
                    if cell.content == CellContent::Mine {
                        mines_exploded += 1;
                    }
                }
                if let CellContent::Number(neighboring_mines) = cell.content {
                    if count_neighboring_flags(self, coord) >= neighboring_mines {
                        iter_neighbors(coord, self.height, self.width)
                            .unwrap()
                            .filter(|&coord| self.get_cell(coord).unwrap().state != CellState::Open)
                            .for_each(|coord| queue.push_back(coord));
                    }
                }
            }
        }
        self.exploded += mines_exploded;
        Ok(OpenResult::new(
            self.get_cell(coord).unwrap(),
            cells_opened,
            mines_exploded,
            flags_touched,
        ))
    }

    fn open_one(&mut self, coord: Coordinate) -> Result<CellContent> {
        self.check_coordinate(coord)?;
        let cell = self.get_cell(coord).unwrap();
        if cell.state == CellState::Closed {
            self.open.insert(coord);
            if cell.content == CellContent::Mine {
                self.exploded += 1;
            }
        }
        Ok(cell.content)
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

    fn get_cell(&self, coord: Coordinate) -> Result<Cell> {
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

    fn height(&self) -> usize {
        self.height
    }

    fn width(&self) -> usize {
        self.width
    }

    fn mines(&self) -> usize {
        self.mines.len()
    }

    fn started_from(&self) -> Coordinate {
        self.start_from
    }

    fn get_game_state(&self) -> GameState {
        GameState {
            opened: self.open.len(),
            flagged: self.flagged.len(),
            mines_left: self.mines.len() - self.flagged.len() - self.exploded,
        }
    }
}

impl Display for MSHash {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        MineSweeper::fmt(self, f)
    }
}
