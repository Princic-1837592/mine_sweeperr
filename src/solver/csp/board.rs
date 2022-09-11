use crate::Coordinate;

const UNKNOWN: i8 = -5;
const CONSTRAINED: i8 = -4;
const MARKED: i8 = -3;
const CLEAR: i8 = 0;

//todo trovare un nome più significativo.
// deve rappresentare la cella di una matrice che l'unico scopo di essere risolta
#[derive(Debug, Clone, Copy)]
struct Cell {
    coordinate: Coordinate,
    state: i8,
    boundary_level: u8,
    test_assignment: i8,
}

impl Cell {
    fn new(coordinate: Coordinate) -> Self {
        Cell {
            coordinate,
            state: UNKNOWN,
            boundary_level: 0,
            test_assignment: -1,
        }
    }
}

pub(crate) struct Board {
    unknown: u32,
    constrained: u32,
    mine: u32,
    clear: u32,
    cells: Vec<Vec<Cell>>,
}

impl Board {
    pub(crate) fn new(height: usize, width: usize) -> Self {
        let mut cells = vec![Vec::with_capacity(width); height];
        for r in 0..height {
            for c in 0..width {
                cells[r].push(Cell::new((r, c)));
            }
        }
        Board {
            unknown: 0,
            constrained: 0,
            mine: 0,
            clear: 0,
            cells,
        }
    }
}
