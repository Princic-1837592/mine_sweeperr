mod mine_sweeper;
mod ms_hash;
mod ms_matrix;
mod utils;


pub use mine_sweeper::*;
pub use ms_hash::*;
pub use ms_matrix::*;


#[cfg(test)]
mod tests {
    use crate::MineSweeper;
    use crate::ms_matrix::GridMatrix;


    #[test]
    fn it_works() {
        let ms: GridMatrix = GridMatrix::new(10, 10, 10);
        println!("{}", ms);
    }
}
