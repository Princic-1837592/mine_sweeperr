#[cfg(test)]
mod test_formatter {
    use crate::{MSMatrix, MineSweeper};

    #[test]
    fn simple_formatter() {
        let start_from = (0, 0);
        let mut ms = MSMatrix::new((5, 5, 5).into(), start_from).unwrap();
        let mut expected = r#"
CCCCC
CCCCC
CCCCC
CCCCC
CCCCC
"#[1..]
            .to_string();
        assert_eq!(expected, format!("{:}", ms));

        ms = MSMatrix::new((5, 11, 5).into(), start_from).unwrap();
        expected = r#"
CCCCCCCCCCC
CCCCCCCCCCC
CCCCCCCCCCC
CCCCCCCCCCC
CCCCCCCCCCC
"#[1..]
            .to_string();
        assert_eq!(expected, format!("{:}", ms));

        ms = MSMatrix::new((11, 12, 5).into(), start_from).unwrap();
        expected = r#"
CCCCCCCCCCCC
CCCCCCCCCCCC
CCCCCCCCCCCC
CCCCCCCCCCCC
CCCCCCCCCCCC
CCCCCCCCCCCC
CCCCCCCCCCCC
CCCCCCCCCCCC
CCCCCCCCCCCC
CCCCCCCCCCCC
CCCCCCCCCCCC
"#[1..]
            .to_string();
        assert_eq!(expected, format!("{:}", ms));
    }

    #[test]
    fn alternate_formatter() {
        let start_from = (0, 0);
        let mut ms = MSMatrix::new((5, 5, 5).into(), start_from).unwrap();
        let mut expected = r#"
🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪
"#[1..]
            .to_string();
        assert_eq!(expected, format!("{:#}", ms));

        ms = MSMatrix::new((5, 11, 5).into(), start_from).unwrap();
        expected = r#"
🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
"#[1..]
            .to_string();
        assert_eq!(expected, format!("{:#}", ms));

        ms = MSMatrix::new((11, 12, 5).into(), start_from).unwrap();
        expected = r#"
🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
"#[1..]
            .to_string();
        assert_eq!(expected, format!("{:#}", ms));
    }

    #[test]
    fn precision_formatter() {
        let start_from = (0, 0);
        let mut ms = MSMatrix::new((5, 5, 5).into(), start_from).unwrap();
        let mut expected = r#"
   01234

0  CCCCC
1  CCCCC
2  CCCCC
3  CCCCC
4  CCCCC
"#[1..]
            .to_string();
        assert_eq!(expected, format!("{:.0}", ms));

        ms = MSMatrix::new((5, 11, 5).into(), start_from).unwrap();
        expected = r#"
             1
   01234567890

0  CCCCCCCCCCC
1  CCCCCCCCCCC
2  CCCCCCCCCCC
3  CCCCCCCCCCC
4  CCCCCCCCCCC
"#[1..]
            .to_string();
        assert_eq!(expected, format!("{:.0}", ms));

        ms = MSMatrix::new((11, 12, 5).into(), start_from).unwrap();
        expected = r#"
              11
    012345678901

 0  CCCCCCCCCCCC
 1  CCCCCCCCCCCC
 2  CCCCCCCCCCCC
 3  CCCCCCCCCCCC
 4  CCCCCCCCCCCC
 5  CCCCCCCCCCCC
 6  CCCCCCCCCCCC
 7  CCCCCCCCCCCC
 8  CCCCCCCCCCCC
 9  CCCCCCCCCCCC
10  CCCCCCCCCCCC
"#[1..]
            .to_string();
        assert_eq!(expected, format!("{:.0}", ms));
    }

    #[test]
    fn full_formatter() {
        let start_from = (0, 0);
        let mut ms = MSMatrix::new((5, 5, 5).into(), start_from).unwrap();
        let mut expected = r#"
🟫  0️⃣1️⃣2️⃣3️⃣4️⃣

0️⃣  🟪🟪🟪🟪🟪
1️⃣  🟪🟪🟪🟪🟪
2️⃣  🟪🟪🟪🟪🟪
3️⃣  🟪🟪🟪🟪🟪
4️⃣  🟪🟪🟪🟪🟪
"#[1..]
            .to_string();
        assert_eq!(expected, format!("{:#.0}", ms));

        ms = MSMatrix::new((5, 11, 5).into(), start_from).unwrap();
        expected = r#"
🟫  🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫1️⃣
🟫  0️⃣1️⃣2️⃣3️⃣4️⃣5️⃣6️⃣7️⃣8️⃣9️⃣0️⃣

0️⃣  🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
1️⃣  🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
2️⃣  🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
3️⃣  🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
4️⃣  🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
"#[1..]
            .to_string();
        assert_eq!(expected, format!("{:#.0}", ms));

        ms = MSMatrix::new((11, 12, 5).into(), start_from).unwrap();
        expected = r#"
🟫🟫  🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫1️⃣1️⃣
🟫🟫  0️⃣1️⃣2️⃣3️⃣4️⃣5️⃣6️⃣7️⃣8️⃣9️⃣0️⃣1️⃣

🟫0️⃣  🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟫1️⃣  🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟫2️⃣  🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟫3️⃣  🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟫4️⃣  🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟫5️⃣  🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟫6️⃣  🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟫7️⃣  🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟫8️⃣  🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟫9️⃣  🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
1️⃣0️⃣  🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
"#[1..]
            .to_string();
        assert_eq!(expected, format!("{:#.0}", ms));
    }
}

#[cfg(test)]
mod test_types {
    use crate::Difficulty;
    #[test]
    fn difficulty() {
        let mut difficulty: Difficulty;

        difficulty = (10, 10, 0.1).into();
        assert_eq!(difficulty, Difficulty::custom(10, 10, 10));

        difficulty = (10, 10, 1.0).into();
        assert_eq!(difficulty, Difficulty::custom(10, 10, 100));

        difficulty = (10, 10, 0.0).into();
        assert_eq!(difficulty, Difficulty::custom(10, 10, 0));

        difficulty = (10, 10, 0.5).into();
        assert_eq!(difficulty, Difficulty::custom(10, 10, 50));
    }
}

#[cfg(test)]
mod test_oters {
    use crate::solver::Solver;
    use crate::{MSMatrix, MineSweeper};
    use std::marker::PhantomData;

    fn return_generic_type() {
        struct Difficulty<S, M>
        where
            S: Solver<M>,
            M: MineSweeper,
        {
            height: usize,
            width: usize,
            mines: usize,
            deterministic: bool,
            phantom: PhantomData<(S, M)>,
        }

        impl<S, M> Difficulty<S, M>
        where
            S: Solver<M>,
            M: MineSweeper,
        {
            const fn new(height: usize, width: usize, mines: usize) -> Self {
                Difficulty {
                    height,
                    width,
                    mines,
                    deterministic: true,
                    phantom: PhantomData,
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

            pub const fn deterministic(self) -> Self {
                Self {
                    deterministic: true,
                    ..self
                }
            }

            pub const fn non_deterministic(self) -> Self {
                Self {
                    deterministic: false,
                    ..self
                }
            }
        }
        let difficulty = Difficulty::easy();
    }
}
