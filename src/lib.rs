use wasm_bindgen::prelude::wasm_bindgen;

extern crate cfg_if;
extern crate wasm_bindgen;
extern crate web_sys;

mod drawer;

#[wasm_bindgen(start)]
pub fn main() {
    drawer::start();
}
