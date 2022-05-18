use crate::{Result, Coordinate, Error::OutOfBounds};


/// Returns an iterator over the neighbors of the given cell.
/// If the coordinates are out of bounds returns [`OutOfBounds`](OutOfBounds).
/// You can safely unwrap the result if you are sure that the given coordinates are in bounds.
pub fn iter_neighbors(coord @ (x, y): Coordinate, height: usize, width: usize) -> Result<impl Iterator<Item = Coordinate>> {
    if x >= height || y >= width {
        Err(OutOfBounds)
    } else {
        Ok(
            (x.saturating_sub(1)..=(x + 1).min(height - 1))
                .flat_map(
                    move |i| {
                        (y.saturating_sub(1)..=(y + 1).min(width - 1))
                            .map(move |j| (i, j))
                    }
                )
                .filter(move |&pos| pos != coord)
        )
    }
}


/// Returns a string representing the superior numbers indicating columns, to be read in vertical.
/// Example: if `width = 15`, returns
/// ```
///           11111
/// 012345678901234
/// ```
pub(crate) fn get_column_numbers(width: usize) -> String {
    let max_digits = (width - 1).to_string().len();
    let mut column_numbers = String::with_capacity(max_digits * (width + 1));
    let mut i = 10_usize.pow((max_digits - 1) as u32);
    while i >= 1 {
        for j in 0..width {
            column_numbers.push_str(&if j >= i || j == 0 && i == 1 { format!("{}", j / i % 10) } else { String::from(' ') });
        }
        column_numbers.push('\n');
        i /= 10;
    }
    column_numbers
}
