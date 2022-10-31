use crate::{Cell, CellContent, CellState, Coordinate, OpenResult};

pub enum TestAction {
    Open(Coordinate, OpenResult),
    Flag(Coordinate, CellState),
}

pub type MSFrom<'a> = (usize, usize, &'a [usize], (usize, usize));

macro_rules! open {
    ($r:expr, $c:expr, $number:expr, $opened:expr) => {
        TestAction::Open(
            ($r, $c),
            OpenResult::new(
                Cell::new(CellState::Open, CellContent::Number($number)),
                $opened,
                0,
            ),
        )
    };
}

macro_rules! flag {
    ($r:expr, $c:expr) => {
        TestAction::Flag(($r, $c), CellState::Flagged)
    };
}

#[allow(clippy::type_complexity)]
pub static OPEN_DATA: &[(MSFrom, &[TestAction])] = &[
    (
        (9, 9, &[4, 6, 15, 16, 19, 47, 51, 68, 70, 74], (0, 0)),
        &[
            open!(0, 0, 0, 8),
            open!(2, 2, 1, 1),
            open!(3, 2, 1, 1),
            open!(3, 1, 1, 1),
            open!(3, 0, 1, 1),
            open!(4, 0, 0, 10),
            open!(6, 2, 1, 1),
            open!(5, 3, 1, 1),
            open!(6, 3, 1, 1),
            open!(7, 3, 1, 1),
            open!(4, 3, 1, 1),
            open!(3, 3, 0, 28),
            flag!(2, 1),
            open!(3, 1, 1, 1),
            flag!(0, 4),
            flag!(1, 6),
            flag!(1, 7),
            open!(2, 7, 2, 3),
            flag!(0, 6),
            open!(0, 5, 3, 1),
            flag!(5, 2),
            open!(6, 3, 1, 2),
            flag!(8, 2),
            open!(7, 3, 1, 2),
            flag!(5, 6),
            flag!(7, 5),
            open!(6, 5, 2, 2),
            flag!(7, 7),
            open!(6, 7, 2, 5),
        ],
    ),
    // (
    //     (9, 9, &[], (0, 0)),
    //     &[TestAction::Open(
    //         (0, 0),
    //         OpenResult::new(Cell::new(CellState::Open, CellContent::Number(0)), 21, 0),
    //     )],
    // ),
];
