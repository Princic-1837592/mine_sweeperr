use std::fmt::Write;

use crate::{CellContent, CellState, Coordinate, Error::OutOfBounds, MineSweeper, Result};

/// Contains emoji numbers from 0 to 9. position 10 is the emoji to represent a 0-cell.
pub(crate) const NUMBERS: [&str; 11] = ["0️⃣", "1️⃣", "2️⃣", "3️⃣", "4️⃣", "5️⃣", "6️⃣", "7️⃣", "8️⃣", "9️⃣", "🟩"];
pub(crate) const ROW_NUMBER_RIGHT_SEPARATOR: &str = "  ";

/// Returns an iterator over the neighbors of the given cell (excluded the cell itself).
/// If the coordinates are out of bounds returns [`OutOfBounds`](OutOfBounds).
/// You can safely unwrap the result if you are sure that the given coordinates are in bounds.
pub fn iter_neighbors(
    coord @ (r, c): Coordinate,
    height: usize,
    width: usize,
) -> Result<impl Iterator<Item = Coordinate>> {
    if r >= height || c >= width {
        Err(OutOfBounds)
    } else {
        Ok((r.saturating_sub(1)..=(r + 1).min(height - 1))
            .flat_map(move |i| (c.saturating_sub(1)..=(c + 1).min(width - 1)).map(move |j| (i, j)))
            .filter(move |&pos| pos != coord))
    }
}

pub(crate) fn get_neighboring_flags(
    ms: &impl MineSweeper,
    coord: Coordinate,
) -> impl Iterator<Item = Coordinate> + '_ {
    iter_neighbors(coord, ms.height(), ms.width())
        .unwrap()
        .filter(|&neighbor| ms.get_cell(neighbor).unwrap().state == CellState::Flagged)
}

pub(crate) fn count_neighboring_flags(ms: &impl MineSweeper, coord: Coordinate) -> u8 {
    get_neighboring_flags(ms, coord).count() as u8
}

pub(crate) fn get_neighboring_mines(
    ms: &impl MineSweeper,
    coord: Coordinate,
) -> impl Iterator<Item = Coordinate> + '_ {
    iter_neighbors(coord, ms.height(), ms.width())
        .unwrap()
        .filter(|&neighbor| ms.get_cell(neighbor).unwrap().content == CellContent::Mine)
}

pub(crate) fn count_neighboring_mines(ms: &impl MineSweeper, coord: Coordinate) -> u8 {
    get_neighboring_mines(ms, coord).count() as u8
}

pub(crate) fn get_neighboring_closed(ms: &impl MineSweeper, coord: Coordinate) -> Vec<Coordinate> {
    iter_neighbors(coord, ms.height(), ms.width())
        .unwrap()
        .filter(|&neighbor| ms.get_cell(neighbor).unwrap().state == CellState::Closed)
        .collect()
}

/// Returns a string representing the superior numbers indicating columns, to be read in vertical.
pub(crate) fn get_column_numbers(height: usize, width: usize, use_emojis: bool) -> String {
    let (max_height_digits, max_width_digits) = (
        (height - 1).to_string().len(),
        (width - 1).to_string().len(),
    );
    // The space to leave on the left considering that will be occupied by row numbers below.
    let left_space = max_height_digits + ROW_NUMBER_RIGHT_SEPARATOR.len();
    // Each line is large: the space taken by row numbers + width + new line.
    // The number of rows for column numbers is max_width_digits.
    // At the end, an extra new line will be added.
    // So the total number of characters is: max_width_digits * (left_space + width + 1) + 1.
    let mut result = String::with_capacity(max_width_digits * (left_space + width + 1) + 1);
    let mut i = 10_usize.pow((max_width_digits - 1) as u32);
    while i >= 1 {
        write!(
            result,
            "{}{}",
            if use_emojis { "🟫" } else { " " }.repeat(max_height_digits),
            ROW_NUMBER_RIGHT_SEPARATOR
        )
        .expect("Failed to write to string");
        for j in 0..width {
            if use_emojis {
                result.push_str(&if j >= i || j == 0 && i == 1 {
                    NUMBERS[j / i % 10].to_string()
                } else {
                    String::from("🟫")
                });
            } else {
                result.push_str(&if j >= i || j == 0 && i == 1 {
                    (j / i % 10).to_string()
                } else {
                    String::from(" ")
                });
            }
        }
        result.push('\n');
        i /= 10;
    }
    result.push('\n');
    result
}

pub(crate) fn get_row_number(number: usize, width: usize, use_emojis: bool) -> String {
    let number = number.to_string();
    let digits = number.len();
    let mut result = String::with_capacity(width);
    write!(
        result,
        "{}",
        String::from(if use_emojis { "🟫" } else { " " }).repeat(width - digits)
    )
    .expect("Failed to write to string");
    for c in number.chars() {
        if use_emojis {
            result.push_str(NUMBERS[c.to_digit(10).unwrap() as usize]);
        } else {
            result.push(c);
        }
    }
    result
}
