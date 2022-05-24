#[cfg(not(target_family = "wasm"))]
pub(crate) use rand::{thread_rng, Rng};


#[cfg(not(target_family = "wasm"))]
pub(crate) fn gen_range(rng: &mut impl Rng, range: std::ops::Range<usize>) -> usize {
    rng.gen_range(range.start..range.end)
}


#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;


#[cfg(target_family = "wasm")]
/// Fake trait to replace the unsupported rand crate when compiling with wasm.
/// in that case, just use [`new`](crate::MineSweeper::new) instead of [`from_rng`](crate::MineSweeper::from_rng).
pub trait Rng {}


#[cfg(target_family = "wasm")]
pub(crate) struct RngWrapper;


#[cfg(target_family = "wasm")]
impl Rng for RngWrapper {}


#[cfg(target_family = "wasm")]
pub(crate) fn thread_rng() -> RngWrapper {
    RngWrapper {}
}


#[cfg(target_family = "wasm")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;
}


#[cfg(target_family = "wasm")]
pub(crate) fn gen_range(_: &mut impl Rng, range: std::ops::Range<usize>) -> usize {
    (random() * (range.end - range.start) as f64).floor() as usize + range.start
}
