extern crate cfg_if;
extern crate wasm_bindgen;
extern crate web_sys;

// graphical pipeline
mod scene;

mod transforms; // matrix for rotation and translation
mod tri3D;
mod vec3D;

mod camera;
mod loader; // stl object loader
