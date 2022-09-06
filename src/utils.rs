use crate::{Coordinate, Error::OutOfBounds, Result};
use std::fmt::Write;

/// Contains emoji numbers from 0 to 9. position 10 is the emoji to represent a 0-cell.
pub(crate) const NUMBERS: [&str; 11] = ["0️⃣", "1️⃣", "2️⃣", "3️⃣", "4️⃣", "5️⃣", "6️⃣", "7️⃣", "8️⃣", "9️⃣", "🟩"];
pub(crate) const ROW_NUMBER_RIGHT_SEPARATOR: &str = "  ";

/// Returns an iterator over the neighbors of the given cell.
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

#[cfg(test)]
mod tests {
    use crate::{get_column_numbers, iter_neighbors};
    use std::collections::HashSet;

    #[test]
    fn neighbors() {
        let (h, w) = (10, 10);
        let mut neighbors: HashSet<_> = iter_neighbors((0, 0), h, w).unwrap().collect();
        assert_eq!(neighbors, HashSet::from([(1, 1), (0, 1), (1, 0)]));

        neighbors = iter_neighbors((h - 1, w - 1), h, w).unwrap().collect();
        assert_eq!(
            neighbors,
            HashSet::from([(h - 2, w - 1), (h - 2, w - 2), (h - 1, w - 2)])
        );

        neighbors = iter_neighbors((h - 1, w - 2), h, w).unwrap().collect();
        assert_eq!(
            neighbors,
            HashSet::from([
                (h - 1, w - 3),
                (h - 2, w - 1),
                (h - 2, w - 3),
                (h - 2, w - 2),
                (h - 1, w - 1)
            ])
        );

        neighbors = iter_neighbors((0, 1), h, w).unwrap().collect();
        assert_eq!(
            neighbors,
            HashSet::from([(1, 0), (0, 2), (0, 0), (1, 1), (1, 2)])
        );

        neighbors = iter_neighbors((1, 1), h, w).unwrap().collect();
        assert_eq!(
            neighbors,
            HashSet::from([
                (1, 2),
                (1, 0),
                (0, 2),
                (0, 0),
                (2, 0),
                (2, 1),
                (2, 2),
                (0, 1)
            ])
        );
    }

    #[test]
    fn test_column_numbers() {
        let mut expected = r#"
🟫  0️⃣1️⃣2️⃣3️⃣4️⃣5️⃣6️⃣7️⃣8️⃣

"#[1..]
            .to_string();
        assert_eq!(expected, get_column_numbers(9, 9, true));

        expected = r#"
   0123456789

"#[1..]
            .to_string();
        assert_eq!(expected, get_column_numbers(10, 10, false));

        expected = r#"
🟫🟫  🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫1️⃣1️⃣1️⃣1️⃣1️⃣
🟫🟫  0️⃣1️⃣2️⃣3️⃣4️⃣5️⃣6️⃣7️⃣8️⃣9️⃣0️⃣1️⃣2️⃣3️⃣4️⃣

"#[1..]
            .to_string();
        assert_eq!(expected, get_column_numbers(15, 15, true));

        expected = r#"
                111111111122222
      0123456789012345678901234

"#[1..]
            .to_string();
        assert_eq!(expected, get_column_numbers(1250, 25, false));

        expected = r#"
🟫🟫  🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫1️⃣1️⃣1️⃣1️⃣1️⃣
🟫🟫  🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫1️⃣1️⃣1️⃣1️⃣1️⃣1️⃣1️⃣1️⃣1️⃣1️⃣2️⃣2️⃣2️⃣2️⃣2️⃣2️⃣2️⃣2️⃣2️⃣2️⃣3️⃣3️⃣3️⃣3️⃣3️⃣3️⃣3️⃣3️⃣3️⃣3️⃣4️⃣4️⃣4️⃣4️⃣4️⃣4️⃣4️⃣4️⃣4️⃣4️⃣5️⃣5️⃣5️⃣5️⃣5️⃣5️⃣5️⃣5️⃣5️⃣5️⃣6️⃣6️⃣6️⃣6️⃣6️⃣6️⃣6️⃣6️⃣6️⃣6️⃣7️⃣7️⃣7️⃣7️⃣7️⃣7️⃣7️⃣7️⃣7️⃣7️⃣8️⃣8️⃣8️⃣8️⃣8️⃣8️⃣8️⃣8️⃣8️⃣8️⃣9️⃣9️⃣9️⃣9️⃣9️⃣9️⃣9️⃣9️⃣9️⃣9️⃣0️⃣0️⃣0️⃣0️⃣0️⃣
🟫🟫  0️⃣1️⃣2️⃣3️⃣4️⃣5️⃣6️⃣7️⃣8️⃣9️⃣0️⃣1️⃣2️⃣3️⃣4️⃣5️⃣6️⃣7️⃣8️⃣9️⃣0️⃣1️⃣2️⃣3️⃣4️⃣5️⃣6️⃣7️⃣8️⃣9️⃣0️⃣1️⃣2️⃣3️⃣4️⃣5️⃣6️⃣7️⃣8️⃣9️⃣0️⃣1️⃣2️⃣3️⃣4️⃣5️⃣6️⃣7️⃣8️⃣9️⃣0️⃣1️⃣2️⃣3️⃣4️⃣5️⃣6️⃣7️⃣8️⃣9️⃣0️⃣1️⃣2️⃣3️⃣4️⃣5️⃣6️⃣7️⃣8️⃣9️⃣0️⃣1️⃣2️⃣3️⃣4️⃣5️⃣6️⃣7️⃣8️⃣9️⃣0️⃣1️⃣2️⃣3️⃣4️⃣5️⃣6️⃣7️⃣8️⃣9️⃣0️⃣1️⃣2️⃣3️⃣4️⃣5️⃣6️⃣7️⃣8️⃣9️⃣0️⃣1️⃣2️⃣3️⃣4️⃣

"#[1..].to_string();
        assert_eq!(expected, get_column_numbers(11, 105, true));
    }
}
