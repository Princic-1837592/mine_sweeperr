#[cfg(test)]
mod tests;

use crate::{
    count_neighboring_flags, get_neighboring_closed, iter_neighbors, CellContent, CellState,
    Coordinate, MineSweeper, Result, Solver,
};
use std::collections::VecDeque;
use std::fmt::Display;

pub struct SPSolver {}

impl SPSolver {
    fn apply(ms: &mut (impl MineSweeper + Display), coord: Coordinate) -> Result<()> {
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
                    neighboring_closed = get_neighboring_closed(ms, cell);
                    neighboring_flags = count_neighboring_flags(ms, cell);
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
        Ok(())
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

impl<M: MineSweeper + Display> Solver<M> for SPSolver {
    fn solve(mut ms: M, start_from: Coordinate) -> Result<bool> {
        SPSolver::apply(&mut ms, start_from)?;
        Ok(ms.get_game_state().opened == ms.width() * ms.height() - ms.mines())
    }
}
