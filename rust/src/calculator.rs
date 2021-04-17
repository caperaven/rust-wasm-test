use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Vector2 {
    pub x: f32,
    pub y: f32
}

#[wasm_bindgen]
impl Vector2 {
    pub fn new(x: f32, y: f32) -> Vector2 {
        return Vector2{x: x, y: y};
    }

    pub fn add(&mut self, v: Vector2) {
        self.x = self.x + v.x;
        self.y = self.y + v.y;
    }
}


