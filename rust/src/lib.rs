mod calculator;

use wasm_bindgen::prelude::*;
use calculator::Vector2;

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[wasm_bindgen]
pub fn add_vectors(v1: Vector2, v2: Vector2) -> Vector2 {
    Vector2 {x: v1.x + v2.x, y: v1.y + v2.y}
}