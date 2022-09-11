use crate::Coordinate;


pub(crate) struct Constraint {
    // variables: Vec<&mut >,
    constant: u8,
    // nvariables: u8,
    unassigned: u8,
    current_constant: u8,
    next_unassigned: bool, //todo
}

impl Constraint {
    fn new() -> Self {
        Constraint {
            // variables: Vec::with_capacity(8),
            constant: 0,
            // nvariables: 0,
            unassigned: 0,
            current_constant: 0,
            next_unassigned: false,
        }
    }

    fn add_var(&mut self, var: Coordinate) {
        // self.variables.push(var);
        // self.nvariables += 1;
    }

    // fn get_count(&self) -> usize {
    //     self.variables.len()
    // }

    fn get_constant(&self) -> u8 {
        self.constant
    }

    // fn is_empty(&self) -> bool {
    //     self.variables.is_empty()
    // }
}
