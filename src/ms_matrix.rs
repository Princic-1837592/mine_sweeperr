use crate::mine_sweeper::{Cell, CellContent, CellState, Coordinate, Error, Result,MineSweeper};
use crate::utils::iter_neighbors;
use rand::Rng;
use std::fmt::{Display, Formatter};


/// A grid using a matrix of cells.
/// Use this when you want to load the whole grid in memory at the beginning.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GridMatrix {
    width: usize,
    height: usize,
    cells: Vec<Vec<Cell>>,
}


impl GridMatrix {
    fn randomize_mines(&mut self, mines: usize) {
        let mut rng = rand::thread_rng();
        let mut mines_left = mines;
        while mines_left > 0 {
            let coord @ (x, y) = (rng.gen_range(0..self.width), rng.gen_range(0..self.height));
            if let CellContent::Number(_) = self.cells[x][y].content {
                self.cells[x][y].content = CellContent::Mine;
                self.increment_neighbors(coord);
                mines_left -= 1;
            }
        }
    }

    fn increment_neighbors(&mut self, coord: Coordinate) {
        iter_neighbors(coord, self.width, self.height).for_each(|coord| {
            if let CellContent::Number(n) = self.cells[coord.0][coord.1].content {
                self.cells[coord.0][coord.1].content = CellContent::Number(n + 1);
            }
        });
    }
}


impl MineSweeper for GridMatrix {
    fn new(width: usize, height: usize, mines: usize) -> Self {
        let cells = vec![vec![Cell::new(); width]; height];
        let mut result = GridMatrix {
            width,
            height,
            cells,
        };
        result.randomize_mines(mines);
        result
    }

    fn open(&mut self, (x, y): Coordinate) -> Result<Option<CellContent>> {
        if x >= self.width || y >= self.height {
            Err(Error::OutOfBounds)
        } else {
            match self.cells[x][y].state {
                CellState::Closed => {
                    self.cells[x][y].state = CellState::Open;
                    if let CellContent::Mine = self.cells[x][y].content {
                        return Ok(Some(CellContent::Mine));
                    }
                    // if let (CellContent::Number(0), = self.cells[x][y].content || iter_neighbors((x, y), self.width, self.height){
                    //
                    // }
                    unimplemented!()
                }
                _ => Ok(None),
            }
        }
    }

    fn toggle_flag(&mut self, (x, y): Coordinate) -> Result<Option<CellState>> {
        todo!()
    }

    fn get_cell(&self, coord: Coordinate) -> Result<Option<Cell>> {
        self.cells
            .get(coord.0)
            .map_or(Err(Error::OutOfBounds), |row| {
                row.get(coord.1)
                   .map_or(Err(Error::OutOfBounds), |cell| Ok(Some(*cell)))
            })
    }
}


impl Display for GridMatrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.cells {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
