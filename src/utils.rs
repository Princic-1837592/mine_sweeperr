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
