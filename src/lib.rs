use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Data {
    pub precio: i32,
    pub prestamo: f32,
    pub anos: i32,
    pub interes: f32,
}

#[wasm_bindgen]
pub fn add(num1: u8, num2: u8) -> u8 {
    let sum = num1 + num2;
    return sum;
}
#[wasm_bindgen]
pub fn mort_calculator(info: Data) -> i32 {
    let hi = info.anos + info.precio;

    return hi;
}
