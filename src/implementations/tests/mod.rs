use std::any::type_name;
use std::fmt::{Debug, Display};

use rand::{rngs::StdRng, thread_rng, Rng, SeedableRng};

use test_data::{MSFrom, TestAction, OPEN_DATA};

use crate::{
    iter_neighbors, CellContent, Difficulty, Error, GameState, MSHash,
    MSMatrix, MineSweeper, Result,
};

mod test_data;

#[test]
// #[allow(unused_variables)]
// #[allow(unused_assignments)]
fn play() {
    fn test<M: MineSweeper + Display>(#[allow(unused)] seed: u64) {
        // let mut rng = StdRng::seed_from_u64(seed);
        let mut rng = thread_rng();

        let difficulty = Difficulty::easy();
        let (h, w, m) = difficulty.into();
        let start_from = (rng.gen_range(0..h), rng.gen_range(0..w));
        let mut ms = M::from_rng(difficulty, start_from, &mut rng).unwrap();

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
        // println!("{}", ms);

        // opens all cells
        for i in 0..h {
            for j in 0..w {
                assert!(ms.open((i, j)).is_ok());
                // println!("{}", ms);
            }
        }
    }

    for seed in 0..1 {
        test::<MSMatrix>(seed);
        test::<MSHash>(seed);
    }
}

#[test]
fn invalid_number_of_mines() {
    fn test<M: MineSweeper>(#[allow(unused)] seed: u64) {
        // let mut rng = StdRng::seed_from_u64(seed);
        let mut rng = thread_rng();

        let (h, w) = (rng.gen_range(4..100), rng.gen_range(4..100));
        let mut m = w * h;
        let mut difficulty = Difficulty::custom(h, w, m);
        let start_from = (rng.gen_range(0..h), rng.gen_range(0..w));

        check_success(M::from_rng(
            difficulty, start_from, &mut rng,
        ));

        m = w * h - 9;
        difficulty = Difficulty::custom(h, w, m);
        check_success(M::from_rng(
            difficulty, start_from, &mut rng,
        ));

        m = w * h - 10;
        difficulty = Difficulty::custom(h, w, m);
        assert!(M::new(difficulty, start_from).is_ok());
    }

    fn check_success<M: MineSweeper>(ms: Result<M>) {
        match ms {
            Err(Error::TooManyMines) => (),
            Err(_) => {
                panic!(
                    "Wrong error: {}::new should panic with Error::TooManyMines!",
                    type_name::<M>()
                )
            }
            Ok(_) => panic!("{}::new should panic!", type_name::<M>()),
        }
    }

    for seed in 0..10 {
        test::<MSMatrix>(seed);
        test::<MSHash>(seed);
    }
}

#[test]
fn start_from() {
    fn test<M: MineSweeper>(#[allow(unused)] seed: u64) {
        // let mut rng = StdRng::seed_from_u64(seed);
        let mut rng = thread_rng();

        let difficulty = Difficulty::hard();
        let (h, w, _) = difficulty.into();
        let start_from = (rng.gen_range(0..h), rng.gen_range(0..w));
        let mut ms: M = M::new(difficulty, start_from).unwrap();

        assert!(
            ms.open(start_from).unwrap().cells_opened
                >= iter_neighbors(start_from, h, w).unwrap().count()
        );

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
    fn test<M: MineSweeper>(#[allow(unused)] seed: u64) {
        // let mut rng = StdRng::seed_from_u64(seed);
        let mut rng = thread_rng();

        let difficulty = Difficulty::hard();
        let (h, w, _) = difficulty.into();
        let start_from = (h, w);

        match M::from_rng(difficulty, start_from, &mut rng) {
            Err(Error::OutOfBounds) => (),
            Err(_) => {
                panic!(
                    "Wrong error: {}::new should panic with Error::OutOfBounds!",
                    type_name::<M>()
                )
            }
            Ok(_) => panic!("{}::new should panic!", type_name::<M>()),
        }

        let start_from = (h - 1, w - 1);
        assert!(M::new(difficulty, start_from).is_ok());
    }

    for seed in 0..10 {
        test::<MSMatrix>(seed);
        test::<MSHash>(seed);
    }
}

#[test]
// #[allow(unused_variables)]
// #[allow(unused_assignments)]
fn compare_implementations() {
    fn test<M1, M2>(seed: u64)
    where
        M1: MineSweeper + Display,
        M2: MineSweeper + Display,
    {
        let mut rng = StdRng::seed_from_u64(seed);
        // let mut rng = thread_rng();

        let difficulty = Difficulty::hard();
        let (h, w, _) = difficulty.into();
        let start_from = (rng.gen_range(0..h), rng.gen_range(0..w));
        let mut ms_1 =
            M1::from_rng(difficulty, start_from, &mut rng.clone()).unwrap();
        let mut ms_2 =
            M2::from_rng(difficulty, start_from, &mut rng.clone()).unwrap();

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

#[test]
fn game_state() {
    fn test<M>(#[allow(unused)] seed: u64)
    where
        M: MineSweeper + Display + Debug,
    {
        // let mut rng = StdRng::seed_from_u64(seed);
        let mut rng = thread_rng();

        let difficulty = Difficulty::easy();
        let (h, w, m) = difficulty.into();
        let start_from = (rng.gen_range(0..h), rng.gen_range(0..w));
        let mut ms = M::from_rng(difficulty, start_from, &mut rng).unwrap();

        assert_eq!(ms.height(), h);
        assert_eq!(ms.width(), w);
        assert_eq!(ms.mines(), m);

        // flags ~60% of the mines
        let (mut flagged, mut mines_left, mut opened) = (0, m, 0);
        for i in 0..h {
            for j in 0..w {
                if let CellContent::Mine = ms.get_cell((i, j)).unwrap().content {
                    if rng.gen_range(0..100) <= 60 {
                        assert!(ms.toggle_flag((i, j)).is_ok());
                        mines_left -= 1;
                        flagged += 1;
                    } else if rng.gen_range(0..100) <= 50 {
                        assert_eq!(ms.open((i, j)).unwrap().mines_exploded, 1);
                        mines_left -= 1;
                        opened += 1;
                    }
                }
            }
        }

        assert_eq!(ms.get_game_state().mines_left, mines_left);
        assert_eq!(ms.get_game_state().flagged, flagged);

        // opens all cells
        let mut open_result;
        for i in 0..h {
            for j in 0..w {
                open_result = ms.open((i, j));
                assert!(open_result.is_ok());

                opened += open_result.unwrap().cells_opened;
                mines_left -= open_result.unwrap().mines_exploded;
                assert_eq!(
                    ms.get_game_state(),
                    GameState {
                        flagged,
                        opened,
                        mines_left
                    }
                );
            }
        }
        assert_eq!(
            ms.get_game_state(),
            GameState {
                flagged,
                opened: h * w - flagged,
                mines_left: 0
            }
        );
    }

    for seed in 0..10 {
        test::<MSMatrix>(seed);
        test::<MSHash>(seed);
    }
}

#[test]
fn open_result() {
    fn test<'a, M>(tuple: MSFrom<'a>, results: &[TestAction])
    where
        M: MineSweeper + Display + From<MSFrom<'a>>,
    {
        let mut ms: M = tuple.into();
        for (i, action) in results.iter().enumerate() {
            match action {
                TestAction::Open(coord, result) => {
                    assert_eq!(
                        ms.open(*coord)
                            .expect("OpenResult should be safe to unwrap"),
                        *result,
                        "{} returned wrong open result at {:?} (action index: {})",
                        type_name::<M>(),
                        coord,
                        i
                    );
                }
                TestAction::Flag(coord, state) => {
                    assert_eq!(
                        ms.toggle_flag(*coord)
                            .expect("CellState should be safe to unwrap"),
                        *state,
                        "{} returned wrong flag result at {:?} (action index: {})",
                        type_name::<M>(),
                        coord,
                        i
                    );
                }
            }
        }
        assert_eq!(
            ms.get_game_state().opened,
            ms.height() * ms.width() - ms.mines()
        );
    }

    for (tuple, results) in OPEN_DATA {
        test::<MSMatrix>(*tuple, results);
    }
}
