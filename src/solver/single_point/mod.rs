use std::collections::VecDeque;

use crate::{
    count_neighboring_flags, get_neighboring_closed, iter_neighbors, CellContent, CellState,
    Coordinate, MineSweeper, Solver,
};

#[cfg(test)]
mod tests;

pub struct SPSolver<M: MineSweeper> {
    ms: M,
}

#[allow(unused)]
impl<M: MineSweeper> SPSolver<M> {
    fn apply(&mut self, coord: Coordinate) -> bool {
        let mut queue = VecDeque::from([coord]);
        let mut cell;
        let mut opened;
        let mut neighboring_flags;
        let mut neighboring_closed;
        let mut add_second_level_neighbors;
        while !queue.is_empty() {
            add_second_level_neighbors = false;
            cell = queue.pop_front().unwrap();
            opened = self.ms.open_one(cell).unwrap();
            if opened == CellContent::Mine {
                break;
            }
            match opened {
                CellContent::Number(cell_number) => {
                    neighboring_closed = get_neighboring_closed(&self.ms, cell);
                    neighboring_flags = count_neighboring_flags(&self.ms, cell);
                    if cell_number == neighboring_flags {
                        queue.extend(neighboring_closed.clone());
                        add_second_level_neighbors = true;
                    } else if neighboring_closed.len() == (cell_number - neighboring_flags) as usize
                    {
                        for &c in &neighboring_closed {
                            self.ms.toggle_flag(c).unwrap();
                        }
                        add_second_level_neighbors = true;
                    }
                    if add_second_level_neighbors {
                        queue.extend(
                            neighboring_closed
                                .iter()
                                .flat_map(|&c| {
                                    iter_neighbors(c, self.ms.height(), self.ms.width()).unwrap()
                                })
                                .filter(|&c| self.ms.get_cell(c).unwrap().state == CellState::Open),
                        );
                    }
                }
                c => unreachable!(
                    "At this point the cell should be a number but found {:?}",
                    c
                ),
            }
        }
        self.ms.get_game_state().opened == self.ms.width() * self.ms.height() - self.ms.mines()
    }

    fn unknowns_near(ms: &impl MineSweeper, (r, c): Coordinate) {
        todo!()
    }

    fn probe_around(ms: &impl MineSweeper, (r, c): Coordinate) {
        todo!()
    }

    fn mark_around(ms: &impl MineSweeper, (r, c): Coordinate) {
        todo!()
    }

    fn adjoin_around(ms: &impl MineSweeper, (r, c): Coordinate) {
        todo!()
    }
}

impl<M: MineSweeper + Clone> Solver<M> for SPSolver<M> {
    fn new(ms: &M) -> Self {
        Self { ms: ms.clone() }
    }

    fn solve(&mut self, start_from: Coordinate) -> bool {
        self.apply(start_from)
    }
}
