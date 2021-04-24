use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn solve(a: f64, b: f64) -> f64 {
    (a.powi(2) + b.powi(2)).sqrt()
}
