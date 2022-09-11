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
    fn apply(ms: &mut (impl MineSweeper + Display), (r, c): Coordinate) {
        let mut queue = VecDeque::from([(r, c)]);
        let mut cell;
        let mut open_result;
        let mut neighboring_flags;
        let mut neighboring_closed;
        let mut add_second_level_neighbors;
        while !queue.is_empty() {
            add_second_level_neighbors = false;
            cell = queue.pop_front().unwrap();
            open_result = ms.open(cell).unwrap();
            // println!("Opened {:?}", open_result);
            println!("{:.0}", ms);
            if open_result.mines_exploded > 0 {
                break;
            }
            match open_result.cell.content {
                CellContent::Number(neighbouring_mines) => {
                    neighboring_closed = get_neighboring_closed(ms, cell);
                    neighboring_flags = count_neighboring_flags(ms, cell);
                    if neighbouring_mines == neighboring_flags {
                        queue.extend(neighboring_closed.clone());
                        add_second_level_neighbors = true;
                    } else if neighboring_closed.len()
                        == (neighbouring_mines - neighboring_flags) as usize
                    {
                        add_second_level_neighbors = true;
                        neighboring_closed.iter().for_each(|&c| {
                            ms.toggle_flag(c).ok();
                        });
                    }
                    if add_second_level_neighbors {
                        queue.extend(
                            neighboring_closed
                                .iter()
                                .flat_map(|&c| iter_neighbors(c, ms.height(), ms.width()).unwrap())
                                /*.filter(|&c| ms.get_cell(c).unwrap().state == CellState::Open)*/,
                        );
                    }
                }
                c => unreachable!(
                    "At this point the cell should be a number but found {:?}",
                    c
                ),
            }
        }
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
    fn new() -> Self {
        todo!()
    }

    fn solve(mut ms: M, start_from: Coordinate) -> Result<bool> {
        // if ms.open(start_from)?.mines_exploded > 0 {
        //     return Ok(false);
        // }
        SPSolver::apply(&mut ms, start_from);
        Ok(ms.get_game_state().opened == ms.width() * ms.height() - ms.mines())
    }
}
