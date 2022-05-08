use crate::mine_sweeper::{Cell, CellContent, CellState, Coordinate, Error, Result, MineSweeper};
use crate::utils::iter_neighbors;
use rand::Rng;
use std::fmt::{Display, Formatter};
use crate::MineSweeperUtils;
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
            .filter(|coord| self.cells[coord.0][coord.1].state == CellState::Flagged)
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
        let cells = vec![vec![Cell::default(); width]; height];
        let mut result = MSMatrix {
            width,
            height,
            cells,
        };
        result.randomize_mines(mines);
        Ok(result)
    }

    fn open(&mut self, coord @ (x, y): Coordinate) -> Result<Option<CellContent>> {
        todo!();
        if x >= self.height || y >= self.width {
            Err(Error::OutOfBounds)
        } else {
            match self.cells[x][y].state {
                CellState::Closed => {
                    let queue = VecDeque::from([coord]);

                    println!("Opening cell at ({}, {})", x, y);
                    self.cells[x][y].state = CellState::Open;
                    if let CellContent::Mine = self.cells[x][y].content {
                        return Ok(Some(CellContent::Mine));
                    }
                    if let CellContent::Number(mines) = self.cells[x][y].content {
                        if mines == self.count_neighboring_flags(coord) {
                            let queue = VecDeque::from(iter_neighbors(coord, self.height, self.width).collect::<Vec<Coordinate>>());
                            while !queue.is_empty(){

                            }
                        }
                    }
                    Ok(Some(self.cells[x][y].content))
                }
                _ => Ok(None),
            }
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

    fn get_cell(&self, (x, y): Coordinate) -> Result<Option<Cell>> {
        self.cells
            .get(x)
            .map_or(
                Err(Error::OutOfBounds),
                |row| row
                    .get(y)
                    .map_or(
                        Err(Error::OutOfBounds),
                        |cell| Ok(Some(*cell)),
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
                    .collect::<Vec<String>>().join(" "))
                .collect::<Vec<String>>().join("\n")
        )?;
        Ok(())
    }
}
