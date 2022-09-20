use std::collections::VecDeque;
use std::fmt::{Display, Formatter};

use rand::Rng;

use crate::{
    check, count_neighboring_flags, iter_neighbors, solver::Solver, Cell, CellContent, CellState,
    Coordinate, Difficulty, Error, GameState, MineSweeper, OpenResult, Result,
};

/// Represents the grid using a matrix of [`cells`](Cell).
/// Use this when you want to load the whole grid in memory at the beginning.
/// Has higher performances when opening cells but takes more memory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MSMatrix {
    height: usize,
    width: usize,
    mines: usize,
    cells: Vec<Vec<Cell>>,
    start_from: Coordinate,
    opened: usize,
    flagged: usize,
    exploded: usize,
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
            opened: 0,
            flagged: 0,
            exploded: 0,
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

    /// Checks the validity of a coordinate.
    fn check_coordinate(&self, (r, c): Coordinate) -> Result<()> {
        if r < self.height && c < self.width {
            Ok(())
        } else {
            Err(Error::OutOfBounds)
        }
    }

    // /// Counts the number of flags around a cell to propagate the opening procedure.
    // fn count_neighboring_flags(&self, coord: Coordinate) -> u8 {
    //     iter_neighbors(coord, self.height, self.width)
    //         .unwrap()
    //         .filter(|(r, c)| self.cells[*r][*c].state == CellState::Flagged)
    //         .count() as u8
    // }
}

impl MineSweeper for MSMatrix {
    fn from_rng<S: Solver>(
        difficulty: Difficulty,
        start_from: Coordinate,
        rng: &mut impl Rng,
    ) -> Result<Self> {
        let difficulty @ (height, width, mines) = difficulty.into();
        check!(difficulty, start_from);
        let mut result;
        loop {
            result = Self::new_unchecked(height, width, mines, start_from);
            result.randomize_mines(mines, start_from, rng);
            if S::solve(&result, start_from).unwrap_or(false) {
                break;
            }
            // println!("Failed to generate a valid board, retrying...");
        }
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
                    if neighboring_mines == 0
                        || count_neighboring_flags(self, coord) >= neighboring_mines
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
        self.opened += cells_opened;
        self.exploded += mines_exploded;
        Ok(OpenResult::new(
            self.cells[r][c],
            cells_opened,
            mines_exploded,
            flags_touched,
        ))
    }

    fn open_one(&mut self, coord @ (r, c): Coordinate) -> Result<CellContent> {
        self.check_coordinate(coord)?;
        if self.cells[r][c].state == CellState::Closed {
            self.cells[r][c].state = CellState::Open;
            self.opened += 1;
            if self.cells[r][c].content == CellContent::Mine {
                self.exploded += 1;
            }
        }
        Ok(self.cells[r][c].content)
    }

    fn toggle_flag(&mut self, coord @ (r, c): Coordinate) -> Result<CellState> {
        self.check_coordinate(coord)?;
        match self.cells[r][c].state {
            CellState::Closed => {
                self.cells[r][c].state = CellState::Flagged;
                self.flagged += 1;
                Ok(CellState::Flagged)
            }
            CellState::Flagged => {
                self.cells[r][c].state = CellState::Closed;
                self.flagged -= 1;
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
        GameState {
            opened: self.opened,
            flagged: self.flagged,
            mines_left: self.mines - self.flagged - self.exploded,
        }
    }
}

impl Display for MSMatrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        MineSweeper::fmt(self, f)
    }
}

impl From<(usize, usize, &[usize], (usize, usize))> for MSMatrix {
    fn from((height, width, mines, start_from): (usize, usize, &[usize], (usize, usize))) -> Self {
        let mut result = Self::new_unchecked(height, width, mines.len(), (0, 0));
        for coord @ (r, c) in mines.iter().map(|&i| (i / width, i % width)) {
            result.cells[r][c].content = CellContent::Mine;
            result.increment_neighbors(coord);
        }
        result.start_from = start_from;
        // for row in result.cells.iter() {
        //     for cell in row.iter() {
        //         print!("{:?} ", cell.content);
        //     }
        //     println!();
        // }
        result
    }
}

impl From<MSMatrix> for (usize, usize, Vec<usize>, (usize, usize)) {
    fn from<'a>(ms: MSMatrix) -> Self {
        let mut mines = Vec::with_capacity(ms.mines);
        for r in 0..ms.height {
            for c in 0..ms.width {
                if ms.cells[r][c].content == CellContent::Mine {
                    mines.push(r * ms.width + c);
                }
            }
        }
        (ms.height, ms.width, mines, ms.start_from)
    }
}
