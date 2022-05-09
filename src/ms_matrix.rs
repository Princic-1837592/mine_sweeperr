use crate::mine_sweeper::{Cell, CellContent, CellState, Coordinate, Error, Result, MineSweeper};
use crate::utils::iter_neighbors;
use rand::Rng;
use std::fmt::{Display, Formatter};
use crate::{MineSweeperUtils, OpenResult};
use std::collections::VecDeque;


/// A grid using a matrix of [`cells`](Cell).
/// Use this when you want to load the whole grid in memory at the beginning.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MSMatrix {
    width: usize,
    height: usize,
    cells: Vec<Vec<Cell>>,
}


impl MineSweeperUtils for MSMatrix {
    fn randomize_mines(&mut self, mines: usize) {
        let mut rng = rand::thread_rng();
        let mut mines_left = mines;
        while mines_left > 0 {
            let coord @ (x, y) = (rng.gen_range(0..self.height), rng.gen_range(0..self.width));
            if let CellContent::Number(_) = self.cells[x][y].content {
                self.cells[x][y].content = CellContent::Mine;
                self.increment_neighbors(coord);
                mines_left -= 1;
            }
        }
    }

    fn increment_neighbors(&mut self, coord: Coordinate) {
        iter_neighbors(coord, self.height, self.width).for_each(|(x, y)| {
            if let CellContent::Number(n) = self.cells[x][y].content {
                self.cells[x][y].content = CellContent::Number(n + 1);
            }
        });
    }

    fn count_neighboring_flags(&self, coord: Coordinate) -> u8 {
        iter_neighbors(coord, self.height, self.width)
            .filter(|(x, y)| self.cells[*x][*y].state == CellState::Flagged)
            .count() as u8
    }

    /// Unimplemented: useless for this implementation.
    fn count_neighboring_mines(&self, _: Coordinate) -> usize {
        unimplemented!()
    }
}


impl MineSweeper for MSMatrix {
    fn new(height: usize, width: usize, mines: usize) -> Result<Self> {
        if mines >= height * width {
            return Err(Error::TooManyMines);
        }
        if width == 0 || height == 0 {
            return Err(Error::InvalidParameters);
        }
        let cells = vec![vec![Cell::default(); width]; height];
        let mut result = MSMatrix {
            width,
            height,
            cells,
        };
        result.randomize_mines(mines);
        Ok(result)
    }

    /// Implements all the additional rules suggested in the [`trait interface`](MineSweeper::open).
    ///
    /// The opening procedure is made using a [`queue`](VecDeque) of [`coordinates`](Coordinate),
    /// in which safe neighboring cells are added when opening a safe cell.
    fn open(&mut self, coord @ (x, y): Coordinate) -> Result<OpenResult> {
        if x >= self.height || y >= self.width {
            Err(Error::OutOfBounds)
        } else {
            let (mut cells_opened, mut mines_found, mut flags_touched) = (0_usize, 0_usize, 0_usize);
            let mut queue = VecDeque::from([coord]);
            while !queue.is_empty() {
                let coord @ (x, y) = queue.pop_front().unwrap();
                match self.cells[x][y].state {
                    CellState::Closed => {
                        self.cells[x][y].state = CellState::Open;
                        cells_opened += 1;
                        if let CellContent::Mine = self.cells[x][y].content {
                            mines_found += 1;
                        }
                        if let CellContent::Number(neighboring_mines) = self.cells[x][y].content {
                            if self.count_neighboring_flags(coord) >= neighboring_mines {
                                iter_neighbors((x, y), self.height, self.width)
                                    .filter(|(x, y)| self.cells[*x][*y].state == CellState::Closed)
                                    .for_each(|coord| queue.push_back(coord));
                            }
                        }
                    }
                    CellState::Flagged => flags_touched += 1,
                    _ => (),
                }
            }
            Ok(OpenResult::new(self.cells[x][y], cells_opened, mines_found, flags_touched))
        }
    }

    fn toggle_flag(&mut self, (x, y): Coordinate) -> Result<Option<CellState>> {
        if x >= self.height || y >= self.width {
            Err(Error::OutOfBounds)
        } else {
            match self.cells[x][y].state {
                CellState::Closed => {
                    self.cells[x][y].state = CellState::Flagged;
                    Ok(Some(CellState::Flagged))
                }
                CellState::Flagged => {
                    self.cells[x][y].state = CellState::Closed;
                    Ok(Some(CellState::Closed))
                }
                _ => Ok(None),
            }
        }
    }

    fn get_cell(&self, (x, y): Coordinate) -> Result<Cell> {
        self.cells
            .get(x)
            .map_or(
                Err(Error::OutOfBounds),
                |row| row
                    .get(y)
                    .map_or(
                        Err(Error::OutOfBounds),
                        |&cell| Ok(cell),
                    ),
            )
    }
}


impl Display for MSMatrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f, "{}",
            self.cells.iter()
                .map(|row| row
                    .iter()
                    .map(|cell| cell.to_string())
                    .collect::<Vec<String>>().join(""))
                .collect::<Vec<String>>().join("\n")
        )?;
        Ok(())
    }
}
