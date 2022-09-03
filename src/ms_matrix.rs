use crate::{
    check, iter_neighbors, Cell, CellContent, CellState, Coordinate, Error, GameState, MineSweeper,
    OpenResult, Result,
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
        height: usize,
        width: usize,
        mines: usize,
        start_from: Coordinate,
        rng: &mut impl Rng,
    ) -> Result<Self> {
        check!(mines height width start_from);
        // if mines >= height * width {
        //     return Err(Error::TooManyMines);
        // }
        // if height == 0 || width == 0 {
        //     return Err(Error::InvalidParameters);
        // }
        // if start_from.0 >= height || start_from.1 >= width {
        //     return Err(Error::OutOfBounds);
        // }
        let mut result = Self::new_unchecked(height, width, mines);
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
                    if self.count_neighboring_flags(coord) >= neighboring_mines {
                        iter_neighbors((r, c), self.height, self.width)
                            .unwrap()
                            .filter(|&(r, c)| self.cells[r][c].state != CellState::Open)
                            .for_each(|coord| queue.push_back(coord));
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

    fn get_game_state(&self) -> GameState {
        todo!()
    }
}

impl Display for MSMatrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        MineSweeper::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use crate::{iter_neighbors, CellContent, Error, MSMatrix, MineSweeper};
    use rand::Rng;

    #[test]
    // #[ignore]
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    fn play_matrix() {
        let mut rng = rand::thread_rng();
        let (h, w, m) = (10, 15, 25);
        let start_from = (rng.gen_range(0..h), rng.gen_range(0..w));
        let mut ms: MSMatrix = MSMatrix::from_rng(h, w, m, start_from, &mut rng).unwrap();
        assert_eq!(ms.height(), h);
        assert_eq!(ms.width(), w);
        assert_eq!(ms.mines(), m);
        for i in 0..h {
            for j in 0..w {
                if let CellContent::Mine = ms.get_cell((i, j)).unwrap().content {
                    if rng.gen_range(0..100) <= 60 {
                        ms.toggle_flag((i, j)).ok();
                    }
                }
            }
        }
        // println!("{:#}\n", ms);
        let mut open_result;
        for i in 0..h {
            for j in 0..w {
                open_result = ms.open((i, j)).unwrap();
                // println!("{:?}", open_result);
                // println!("{}\n\n", ms);
            }
        }
        // println!("{:#}\n", ms);
    }

    #[test]
    fn invalid_number_of_mines() {
        let (h, w) = (10, 15);
        let m = w * h - 9;
        match MSMatrix::new(h, w, m, (0, 0)) {
            Err(Error::TooManyMines) => (),
            Err(_) => panic!("Wrong error: MSMatrix::new should panic as Error::TooManyMines!"),
            Ok(_) => panic!("MSMatrix::new should panic!"),
        }
        let m = w * h - 10;
        assert!(MSMatrix::new(h, w, m, (0, 0)).is_ok());
    }

    #[test]
    fn start_from() {
        for _ in 0..1000 {
            let mut rng = rand::thread_rng();
            let (h, w, m) = (100, 150, 250);
            let start_from = (rng.gen_range(0..h), rng.gen_range(0..w));
            let ms: MSMatrix = MSMatrix::new(h, w, m, start_from).unwrap();
            let mut should_be_safe = iter_neighbors(start_from, h, w)
                .unwrap()
                .map(|(r, c)| ms.get_cell((r, c)).unwrap().content)
                .collect::<Vec<_>>();
            should_be_safe.push(ms.get_cell(start_from).unwrap().content);
            assert_eq!(
                should_be_safe[should_be_safe.len() - 1],
                CellContent::Number(0)
            );
            assert!(!should_be_safe.contains(&CellContent::Mine));
        }
    }

    #[test]
    fn invalid_start_from() {
        let (h, w, m) = (10, 15, 25);
        let start_from = (h, w);
        match MSMatrix::new(h, w, m, start_from) {
            Err(Error::OutOfBounds) => (),
            Err(_) => panic!("Wrong error: MSMatrix::new should panic as Error::OutOfBounds!"),
            Ok(_) => panic!("MSMatrix::new should panic!"),
        }
        let start_from = (h - 1, w - 1);
        assert!(MSMatrix::new(h, w, m, start_from).is_ok());
    }
}
