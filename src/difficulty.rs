/// TODO Represents the difficulty of a game in terms of height, width and number of mines.
///
/// When calling [`MineSweeper::new`](MineSweeper::new) or [`MineSweeper::from_rng`](MineSweeper::from_rng)
/// you can either pass a default difficulty or a custom one.
///
/// The default difficulties are:
/// - `Easy`: `9x9` grid with `10` mines
/// - `Medium`: `16x16` grid with `40` mines
/// - `Hard`: `16x30` grid with `99` mines
///
/// Difficulty can be derived from a tuple representing `(height, width, mines)`
/// or from a tuple representing `(height, width, density)`.
/// For example:
/// ```
/// # use mine_sweeperr::Difficulty;
/// let difficulty: Difficulty = (10, 10, 0.1).into();
/// ```
/// will produce a difficulty with `10x10` grid and `10` mines.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Difficulty {
    height: usize,
    width: usize,
    mines: usize,
}

impl Difficulty {
    const fn new(height: usize, width: usize, mines: usize) -> Self {
        Difficulty {
            height,
            width,
            mines,
        }
    }

    pub const fn easy() -> Self {
        Self::new(9, 9, 10)
    }

    pub const fn medium() -> Self {
        Self::new(16, 16, 40)
    }

    pub const fn hard() -> Self {
        Self::new(16, 30, 99)
    }

    pub const fn custom(height: usize, width: usize, mines: usize) -> Self {
        Self::new(height, width, mines)
    }

    pub fn from_density(height: usize, width: usize, density: f32) -> Self {
        Self::new(height, width, ((height * width) as f32 * density) as usize)
    }
}

impl From<Difficulty> for (usize, usize, usize) {
    fn from(difficulty: Difficulty) -> (usize, usize, usize) {
        (difficulty.height, difficulty.width, difficulty.mines)
    }
}

impl From<(usize, usize, usize)> for Difficulty {
    fn from((height, width, mines): (usize, usize, usize)) -> Difficulty {
        Difficulty::custom(height, width, mines)
    }
}

impl From<(usize, usize, f32)> for Difficulty {
    fn from((height, width, density): (usize, usize, f32)) -> Difficulty {
        Difficulty::from_density(height, width, density)
    }
}
