use crate::{
    check, iter_neighbors, Cell, CellContent, CellState, Coordinate, Error, GameState, MineSweeper,
    OpenResult, Result,
};
use rand::Rng;
use std::collections::{HashSet, VecDeque};
use std::fmt::{Display, Formatter};

/// Represents a grid using [`HashSets`](HashSet) of [`Coordinates`](Coordinate).
/// Use this when you don't want to load the whole grid in memory at the beginning.
/// Has lower performances when opening cells but takes less memory.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MSHash {
    height: usize,
    width: usize,
    open: HashSet<Coordinate>,
    flagged: HashSet<Coordinate>,
    mines: HashSet<Coordinate>,
}

impl MSHash {
    /// Creates a new instance.
    fn new_unchecked(height: usize, width: usize, mines: usize) -> Self {
        Self {
            height,
            width,
            open: Default::default(),
            flagged: Default::default(),
            mines: HashSet::with_capacity(mines),
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

    /// Counts the number of flags around a cell to propagate the opening procedure.
    fn count_neighboring_flags(&self, coord: Coordinate) -> u8 {
        iter_neighbors(coord, self.height, self.width)
            .unwrap()
            .filter(|coord| self.flagged.contains(coord))
            .count() as u8
    }
}

impl MineSweeper for MSHash {
    fn from_rng(
        height: usize,
        width: usize,
        mines: usize,
        start_from: Coordinate,
        rng: &mut impl Rng,
    ) -> Result<Self> {
        check!(mines height width start_from);
        let mut result = Self::new_unchecked(height, width, mines);
        result.randomize_mines(mines, start_from, rng);
        Ok(result)
    }

    /// Implements all the additional rules suggested in the [trait interface](MineSweeper::open).
    ///
    /// The opening procedure is made using a [queue](VecDeque) (not recursive).
    fn open(&mut self, coord: Coordinate) -> Result<OpenResult> {
        self.check_coordinate(coord)?;
        let (mut cells_opened, mut mines_exploded, mut flags_touched) = (0_usize, 0_usize, 0_usize);
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
                    if self.count_neighboring_flags(coord) >= neighboring_mines {
                        iter_neighbors(coord, self.height, self.width)
                            .unwrap()
                            .filter(|&coord| self.get_cell(coord).unwrap().state != CellState::Open)
                            .for_each(|coord| queue.push_back(coord));
                    }
                }
            }
        }
        Ok(OpenResult::new(
            self.get_cell(coord).unwrap(),
            cells_opened,
            mines_exploded,
            flags_touched,
        ))
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

    fn get_game_state(&self) -> GameState {
        todo!()
    }
}

impl Display for MSHash {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        MineSweeper::fmt(self, f)
    }
}
#[cfg(test)]
mod tests {
    use crate::{iter_neighbors, CellContent, Error, MSHash, MineSweeper};
    use rand::Rng;

    #[test]
    // #[ignore]
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    fn play_hash() {
        let mut rng = rand::thread_rng();
        let (h, w, m) = (10, 15, 25);
        let mut ms: MSHash =
            MSHash::new(h, w, m, (rng.gen_range(0..h), rng.gen_range(0..w))).unwrap();
        assert_eq!(ms.height(), h);
        assert_eq!(ms.width(), w);
        assert_eq!(ms.mines(), m);
        for i in 0..h {
            for j in 0..w {
                if let CellContent::Mine = ms.get_cell((i, j)).unwrap().content {
                    if rng.gen_range(0..100) <= 10 {
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
    }

    #[test]
    fn invalid_number_of_mines() {
        let (h, w) = (10, 15);
        let m = w * h;
        match MSHash::new(h, w, m, (0, 0)) {
            Err(Error::TooManyMines) => (),
            _ => panic!("MSMatrix::new should panic here!"),
        }
        let m = w * h - 10;
        assert!(MSHash::new(h, w, m, (0, 0)).is_ok());
    }

    #[test]
    fn start_from() {
        for _ in 0..1000 {
            let mut rng = rand::thread_rng();
            let (h, w, m) = (100, 150, 250);
            let start_from = (rng.gen_range(0..h), rng.gen_range(0..w));
            let ms: MSHash = MSHash::new(h, w, m, start_from).unwrap();
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
        match MSHash::new(h, w, m, start_from) {
            Err(Error::OutOfBounds) => (),
            Err(_) => panic!("Wrong error: MSMatrix::new should panic as Error::OutOfBounds!"),
            Ok(_) => panic!("MSMatrix::new should panic!"),
        }
        let start_from = (h - 1, w - 1);
        assert!(MSHash::new(h, w, m, start_from).is_ok());
    }
}
