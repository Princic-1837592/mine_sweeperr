mod test_formatter {
    use crate::{solver::NonDeterministic, MSMatrix};

    #[test]
    fn simple_formatter() {
        let start_from = (0, 0);
        let mut ms = MSMatrix::new::<NonDeterministic>((5, 5, 5).into(), start_from).unwrap();
        let mut expected = r#"
CCCCC
CCCCC
CCCCC
CCCCC
CCCCC
"#[1..]
            .to_string();
        assert_eq!(expected, format!("{:}", ms));

        ms = MSMatrix::new::<NonDeterministic>((5, 11, 5).into(), start_from).unwrap();
        expected = r#"
CCCCCCCCCCC
CCCCCCCCCCC
CCCCCCCCCCC
CCCCCCCCCCC
CCCCCCCCCCC
"#[1..]
            .to_string();
        assert_eq!(expected, format!("{:}", ms));

        ms = MSMatrix::new::<NonDeterministic>((11, 12, 5).into(), start_from).unwrap();
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
        let mut ms = MSMatrix::new::<NonDeterministic>((5, 5, 5).into(), start_from).unwrap();
        let mut expected = r#"
🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪
"#[1..]
            .to_string();
        assert_eq!(expected, format!("{:#}", ms));

        ms = MSMatrix::new::<NonDeterministic>((5, 11, 5).into(), start_from).unwrap();
        expected = r#"
🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪
"#[1..]
            .to_string();
        assert_eq!(expected, format!("{:#}", ms));

        ms = MSMatrix::new::<NonDeterministic>((11, 12, 5).into(), start_from).unwrap();
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
        let mut ms = MSMatrix::new::<NonDeterministic>((5, 5, 5).into(), start_from).unwrap();
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

        ms = MSMatrix::new::<NonDeterministic>((5, 11, 5).into(), start_from).unwrap();
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

        ms = MSMatrix::new::<NonDeterministic>((11, 12, 5).into(), start_from).unwrap();
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
        let mut ms = MSMatrix::new::<NonDeterministic>((5, 5, 5).into(), start_from).unwrap();
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

        ms = MSMatrix::new::<NonDeterministic>((5, 11, 5).into(), start_from).unwrap();
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

        ms = MSMatrix::new::<NonDeterministic>((11, 12, 5).into(), start_from).unwrap();
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

mod utils {
    use std::collections::HashSet;

    use crate::{get_column_numbers, iter_neighbors};

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
