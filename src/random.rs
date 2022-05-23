#[cfg(not(target_family = "wasm"))]
pub(crate) use rand::{thread_rng, Rng};


pub(crate) fn gen_range(rng: &mut impl Rng, min: usize, max: usize) -> usize {
    rng.gen_range(min..max)
}


#[cfg(target_family = "wasm")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;
}


#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;


#[cfg(target_family = "wasm")]
pub trait Rng {
    fn gen_range(&mut self, range: std::ops::Range<usize>) -> usize {
        (random() * (max - min) as f64).floor() as usize + min
    }
}


#[cfg(target_family = "wasm")]
pub(crate) struct RngWrapper;


#[cfg(target_family = "wasm")]
impl Rng for RngWrapper {}


#[cfg(target_family = "wasm")]
pub(crate) fn thread_rng() -> RngWrapper {
    RngWrapper {}
}
