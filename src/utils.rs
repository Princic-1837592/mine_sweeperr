use crate::mine_sweeper::Coordinate;


pub(crate) fn iter_neighbors((x, y): Coordinate, height: usize, width: usize) -> impl Iterator<Item = Coordinate> {
    (x.saturating_sub(1)..=(x + 1).min(width - 1))
        .flat_map(
            move |i| {
                (y.saturating_sub(1)..=(y + 1).min(height - 1))
                    .map(move |j| (i, j))
            }
        )
        .filter(move |&pos| pos != (x, y))
}
