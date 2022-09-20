use std::collections::VecDeque;
use std::fmt::Display;

use crate::{
    count_neighboring_flags, get_neighboring_closed, iter_neighbors, CellContent, CellState,
    Coordinate, MineSweeper, Result, Solver,
};

#[cfg(test)]
mod tests;

pub struct SPSolver {}

impl SPSolver {
    fn apply(mut ms: impl MineSweeper, coord: Coordinate) -> Result<bool> {
        let mut queue = VecDeque::from([coord]);
        let mut cell;
        let mut opened;
        let mut neighboring_flags;
        let mut neighboring_closed;
        let mut add_second_level_neighbors;
        while !queue.is_empty() {
            add_second_level_neighbors = false;
            cell = queue.pop_front().unwrap();
            opened = ms.open_one(cell).unwrap();
            if opened == CellContent::Mine {
                break;
            }
            match opened {
                CellContent::Number(cell_number) => {
                    neighboring_closed = get_neighboring_closed(&ms, cell);
                    neighboring_flags = count_neighboring_flags(&ms, cell);
                    if cell_number == neighboring_flags {
                        queue.extend(neighboring_closed.clone());
                        add_second_level_neighbors = true;
                    } else if neighboring_closed.len() == (cell_number - neighboring_flags) as usize
                    {
                        for &c in &neighboring_closed {
                            ms.toggle_flag(c)?;
                        }
                        add_second_level_neighbors = true;
                    }
                    if add_second_level_neighbors {
                        queue.extend(
                            neighboring_closed
                                .iter()
                                .flat_map(|&c| iter_neighbors(c, ms.height(), ms.width()).unwrap())
                                .filter(|&c| ms.get_cell(c).unwrap().state == CellState::Open),
                        );
                    }
                }
                c => unreachable!(
                    "At this point the cell should be a number but found {:?}",
                    c
                ),
            }
        }
        Ok(ms.get_game_state().opened == ms.width() * ms.height() - ms.mines())
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

impl Solver for SPSolver {
    fn solve<M: MineSweeper + Clone>(ms: &M, start_from: Coordinate) -> Result<bool> {
        SPSolver::apply(ms.clone(), start_from)
    }
}
