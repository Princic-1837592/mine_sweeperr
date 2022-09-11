use crate::{
    iter_neighbors, CellContent, Difficulty, Error, MSHash, MSMatrix, MineSweeper, NonDeterministic,
};
use rand::{rngs::StdRng, thread_rng, Rng, SeedableRng};
use std::fmt::Display;

#[test]
#[allow(unused_variables)]
#[allow(unused_assignments)]
fn play() {
    fn test<T: MineSweeper + Display>(_seed: u64) {
        // let mut rng = StdRng::seed_from_u64(seed);
        let mut rng = thread_rng();

        let difficulty = Difficulty::medium();
        let (h, w, m) = difficulty.into();
        let start_from = (rng.gen_range(0..h), rng.gen_range(0..w));
        let mut ms = T::from_rng::<NonDeterministic, _>(difficulty, start_from, &mut rng).unwrap();

        assert_eq!(ms.height(), h);
        assert_eq!(ms.width(), w);
        assert_eq!(ms.mines(), m);

        // flags 60% of the mines
        for i in 0..h {
            for j in 0..w {
                if let CellContent::Mine = ms.get_cell((i, j)).unwrap().content {
                    if rng.gen_range(0..100) <= 60 {
                        assert!(ms.toggle_flag((i, j)).is_ok());
                    }
                }
            }
        }

        // opens all cells
        for i in 0..h {
            for j in 0..w {
                assert!(ms.open((i, j)).is_ok());
            }
        }
    }

    for seed in 0..10 {
        test::<MSMatrix>(seed);
        test::<MSHash>(seed);
    }
}

#[test]
fn invalid_number_of_mines() {
    fn test<T: MineSweeper>(_seed: u64) {
        // let mut rng = StdRng::seed_from_u64(_seed);
        let mut rng = thread_rng();

        let (h, w) = (rng.gen_range(1..100), rng.gen_range(1..100));
        let mut m = w * h;
        let mut difficulty = Difficulty::custom(h, w, m);
        let start_from = (rng.gen_range(0..h), rng.gen_range(0..w));

        match T::from_rng::<NonDeterministic, _>(difficulty, start_from, &mut rng) {
            Err(Error::TooManyMines) => (),
            Err(_) => {
                panic!("Wrong error: MineSweeper::new should panic with Error::TooManyMines!")
            }
            Ok(_) => panic!("MineSweeper::new should panic!"),
        }

        m = w * h - 9;
        difficulty = Difficulty::custom(h, w, m);
        match T::from_rng::<NonDeterministic, _>(difficulty, start_from, &mut rng) {
            Err(Error::TooManyMines) => (),
            Err(_) => {
                panic!("Wrong error: MineSweeper::new should panic with Error::TooManyMines!")
            }
            Ok(_) => panic!("MineSweeper::new should panic!"),
        }

        m = w * h - 10;
        difficulty = Difficulty::custom(h, w, m);
        assert!(T::new::<NonDeterministic>(difficulty, start_from).is_ok());
    }

    for seed in 0..10 {
        test::<MSMatrix>(seed);
        test::<MSHash>(seed);
    }
}

#[test]
fn start_from() {
    fn test<T: MineSweeper>(_seed: u64) {
        // let mut rng = StdRng::seed_from_u64(_seed);
        let mut rng = thread_rng();

        let difficulty = Difficulty::hard();
        let (h, w, _) = difficulty.into();
        let start_from = (rng.gen_range(0..h), rng.gen_range(0..w));
        let ms: T = T::new::<NonDeterministic>(difficulty, start_from).unwrap();

        let mut should_be_safe = iter_neighbors(start_from, h, w)
            .unwrap()
            .map(|(r, c)| ms.get_cell((r, c)).unwrap().content);

        assert_eq!(
            ms.get_cell(start_from).unwrap().content,
            CellContent::Number(0)
        );
        assert!(!should_be_safe.any(|cell_content| cell_content == CellContent::Mine));
    }

    for seed in 0..10 {
        test::<MSMatrix>(seed);
        test::<MSHash>(seed);
    }
}

#[test]
fn invalid_start_from() {
    fn test<T: MineSweeper>(_seed: u64) {
        // let mut rng = StdRng::seed_from_u64(_seed);
        let mut rng = thread_rng();

        let difficulty = Difficulty::hard();
        let (h, w, _) = difficulty.into();
        let start_from = (h, w);

        match T::from_rng::<NonDeterministic, _>(difficulty, start_from, &mut rng) {
            Err(Error::OutOfBounds) => (),
            Err(_) => {
                panic!("Wrong error: MineSweeper::new should panic with Error::OutOfBounds!")
            }
            Ok(_) => panic!("MineSweeper::new should panic!"),
        }

        let start_from = (h - 1, w - 1);
        assert!(T::new::<NonDeterministic>(difficulty, start_from).is_ok());
    }

    for seed in 0..10 {
        test::<MSMatrix>(seed);
        test::<MSHash>(seed);
    }
}

#[test]
#[allow(unused_variables)]
#[allow(unused_assignments)]
fn compare_implementations() {
    fn test<T, E>(_seed: u64)
    where
        T: MineSweeper + Display,
        E: MineSweeper + Display,
    {
        let mut rng = StdRng::seed_from_u64(_seed);
        // let mut rng = thread_rng();

        let difficulty = Difficulty::custom(10, 15, 25);
        let (h, w, m) = difficulty.into();
        let start_from = (rng.gen_range(0..h), rng.gen_range(0..w));
        let mut ms_1 =
            T::from_rng::<NonDeterministic, _>(difficulty, start_from, &mut rng.clone()).unwrap();
        let mut ms_2 =
            E::from_rng::<NonDeterministic, _>(difficulty, start_from, &mut rng.clone()).unwrap();

        assert_eq!(ms_1.to_string(), ms_2.to_string());

        // compares the raw content of all cells between the two implementations
        // and flags 5% of the mines, comparing again
        for i in 0..h {
            for j in 0..w {
                assert_eq!(ms_1.get_cell((i, j)), ms_2.get_cell((i, j)));
                if let CellContent::Mine = ms_1.get_cell((i, j)).unwrap().content {
                    if rng.gen_range(0..100) <= 5 {
                        assert_eq!(ms_1.toggle_flag((i, j)), ms_2.toggle_flag((i, j)));
                    }
                }
            }
        }
        assert_eq!(format!("{:#}", ms_1), format!("{:#}", ms_2));

        let (mut ms_1_open, mut ms_2_open);
        // opening the whole grid and comparing strings could take a lot of time for big grids
        // or when the grid has a lot of flags
        for i in 0..h {
            for j in 0..w {
                ms_1_open = ms_1.open((i, j)).unwrap();
                ms_2_open = ms_2.open((i, j)).unwrap();
                assert_eq!(ms_1_open, ms_2_open);
                assert_eq!(format!("{:#}", ms_1), format!("{:#}", ms_2));
            }
        }
    }

    for seed in 0..1 {
        test::<MSMatrix, MSHash>(seed);
    }
}
