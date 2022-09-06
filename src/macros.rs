#[macro_export]
macro_rules! check {
    ($difficulty:ident, $start_from:ident) => {
        if $difficulty.2 >= $difficulty.0 * $difficulty.1 - 9 {
            return Err(Error::TooManyMines);
        }
        if $difficulty.0 == 0 || $difficulty.1 == 0 {
            return Err(Error::InvalidParameters);
        }
        if $start_from.0 >= $difficulty.0 || $start_from.1 >= $difficulty.1 {
            return Err(Error::OutOfBounds);
        }
    };
}
