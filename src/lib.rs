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
        let mut ms: GridMatrix = GridMatrix::new(10, 10, 10);
        println!("{}\n", ms);
        ms.open((0, 0)).expect("Failed to open cell");
        ms.open((0, 1)).unwrap();
        ms.open((0, 2)).unwrap();
        ms.open((0, 3)).unwrap();
        ms.open((0, 4)).unwrap();
        ms.open((0, 5)).unwrap();
        ms.open((0, 6)).unwrap();
        ms.open((0, 7)).unwrap();
        ms.open((0, 8)).unwrap();
        ms.open((0, 9)).unwrap();
        println!("{}\n", ms);
    }
}
