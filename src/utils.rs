use crate::mine_sweeper::Coordinate;


pub(crate) fn iter_neighbors(coord @ (x, y): Coordinate, height: usize, width: usize) -> impl Iterator<Item = Coordinate> {
    if x >= height || y >= width {
        unreachable!("Trying to iter neighbors out of bounds");
    }
    (x.saturating_sub(1)..=(x + 1).min(height - 1))
        .flat_map(
            move |i| {
                (y.saturating_sub(1)..=(y + 1).min(width - 1))
                    .map(move |j| (i, j))
            }
        )
        .filter(move |&pos| pos != coord)
}
