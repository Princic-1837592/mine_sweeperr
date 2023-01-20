use std::{
    collections::VecDeque,
    fmt::{Display, Formatter},
};

use rand::{seq::SliceRandom, Rng};

use crate::{
    check, count_neighboring_flags, count_neighboring_mines, iter_neighbors, solver,
    solver::{NonDeterministic, Solver},
    Cell, CellContent, CellState, Coordinate, Difficulty, Error, GameState, MineSweeper,
    OpenResult, Result,
};

// const MAX_SHUFFLE: usize = 10;

/// Represents the grid using a matrix of [`cells`](Cell).
/// Use this when you want to load the whole grid in memory at the beginning.
/// Has better performances when opening cells but takes more memory.
///
/// # Solver
/// This implementation supports passing a [`Solver`](Solver)
/// to both the constructors. if you use the trait constructors
/// ([`new`](MineSweeper::new) and [`from_rng`](MineSweeper::from_rng))
/// to create an instance of this struct,
/// the [default solver](NonDeterministic) will be used.
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
    seed: u64,
}

impl MSMatrix {
    /// Creates a new instance of the game with the given solver and the default rng ([`thread_rng`](rand::thread_rng)).
    pub fn new<S: Solver<Self>>(difficulty: Difficulty, start_from: Coordinate) -> Result<Self> {
        Self::from_rng::<S>(difficulty, start_from, &mut rand::thread_rng())
    }

    /// Creates a new instance of the game with the given solver and the given rng.
    pub fn from_rng<S: Solver<Self>>(
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
            let mut solver = S::new(&result);
            if solver.solve(start_from) {
                break;
            }
        }
        Ok(result)
    }

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
            seed: 0,
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

    fn decrement_neighbors(&mut self, coord: Coordinate) {
        iter_neighbors(coord, self.height, self.width)
            .unwrap()
            .for_each(|(r, c)| {
                if let CellContent::Number(n) = self.cells[r][c].content {
                    self.cells[r][c].content = CellContent::Number(n - 1);
                }
            });
    }

    fn extract_mine(&mut self, coord @ (r, c): Coordinate) {
        self.decrement_neighbors(coord);
        self.cells[r][c].content = CellContent::Number(count_neighboring_mines(self, coord));
    }

    #[allow(unused)]
    fn swap_cells(&mut self, old_mine @ (r1, c1): Coordinate, new_mine @ (r2, c2): Coordinate) {
        if cfg!(test) {
            // println!("Swapping cells {:?} and {:?}", old_mine, new_mine);
            assert_eq!(self.cells[r1][c1].content, CellContent::Mine);
            assert_ne!(self.cells[r2][c2].content, CellContent::Mine);
        }
        self.extract_mine(old_mine);
        self.cells[r2][c2].content = CellContent::Mine;
        self.increment_neighbors(new_mine);
    }

    #[allow(unused)]
    fn shuffle(&mut self, clusters: Vec<Vec<Coordinate>>, rng: &mut impl Rng) {
        for cluster in clusters {
            let mut from_mine;
            loop {
                from_mine = *cluster.choose(rng).unwrap();
                if self.cells[from_mine.0][from_mine.1].content == CellContent::Mine {
                    break;
                }
            }
            let mut to_cell;
            loop {
                to_cell = *cluster.choose(rng).unwrap();
                if self.cells[to_cell.0][to_cell.1].content != CellContent::Mine {
                    break;
                }
            }
            self.swap_cells(from_mine, to_cell);
        }
    }

    #[cfg(test)]
    #[allow(unused)]
    fn print_raw(&self) {
        for row in &self.cells {
            for cell in row {
                print!("{} ", cell.content);
            }
            println!();
        }
        println!("\n");
    }
}

impl MineSweeper for MSMatrix {
    fn from_rng(
        difficulty: Difficulty,
        start_from: Coordinate,
        rng: &mut impl Rng,
    ) -> Result<Self> {
        Self::from_rng::<NonDeterministic>(difficulty, start_from, rng)
    }

    /// Implements all the additional rules suggested in the [trait interface](MineSweeper::open).
    ///
    /// The opening procedure is made using a [queue](VecDeque) (not recursive).
    fn open(&mut self, coord @ (r, c): Coordinate) -> Result<OpenResult> {
        self.check_coordinate(coord)?;
        let (mut cells_opened, mut mines_exploded) = (0, 0);
        let mut queue = VecDeque::from([coord]);
        while !queue.is_empty() {
            let coord @ (r, c) = queue.pop_front().unwrap();
            if self.cells[r][c].state != CellState::Flagged {
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

#[cfg(test)]
impl From<(usize, usize, &[usize], (usize, usize))> for MSMatrix {
    fn from((height, width, mines, start_from): (usize, usize, &[usize], (usize, usize))) -> Self {
        let mut result = Self::new_unchecked(height, width, mines.len(), (0, 0));
        for coord @ (r, c) in mines.iter().map(|&i| (i / width, i % width)) {
            result.cells[r][c].content = CellContent::Mine;
            result.increment_neighbors(coord);
        }
        result.start_from = start_from;
        result
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use rand::{rngs::StdRng, thread_rng, SeedableRng};

    use crate::{solver::CSPSolver, Coordinate, Difficulty, MSMatrix};

    type MSFrom<'a> = (usize, usize, &'a [usize], (usize, usize));

    #[allow(clippy::type_complexity)]
    static SWAP_DATA: &[(MSFrom, &[(Coordinate, Coordinate, MSFrom)])] = &[(
        (9, 9, &[4, 6, 15, 16, 19, 47, 51, 68, 70, 74], (0, 0)),
        &[
            (
                (2, 1),
                (0, 2),
                (9, 9, &[2, 4, 6, 15, 16, 47, 51, 68, 70, 74], (0, 0)),
            ),
            (
                (7, 7),
                (3, 8),
                (9, 9, &[2, 4, 6, 15, 16, 35, 47, 51, 68, 74], (0, 0)),
            ),
            (
                (5, 6),
                (5, 1),
                (9, 9, &[2, 4, 6, 15, 16, 35, 46, 47, 68, 74], (0, 0)),
            ),
        ],
    )];

    #[test]
    fn swap_mines() {
        for (starting_point, swaps) in SWAP_DATA {
            let mut ms: MSMatrix = (*starting_point).into();
            for (from, to, result) in *swaps {
                ms.swap_cells(*from, *to);
                assert_eq!(ms, (*result).into());
            }
        }
    }

    #[test]
    #[allow(unused)]
    fn smart_generation() {
        let n = 1000;
        for i in 0..n {
            // let mut rng = StdRng::seed_from_u64(i);
            let mut rng = thread_rng();
            // let difficulty = Difficulty::custom(100, 100, 2000);
            let difficulty = Difficulty::medium();
            // let difficulty = Difficulty::hard();
            let ms = MSMatrix::from_rng::<CSPSolver>(difficulty, (0, 0), &mut rng);
        }
    }
}
