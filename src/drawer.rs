use std::f64;
use wasm_bindgen::JsCast;
use web_sys::console;

pub fn start() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("smiley_canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let mut posx = 20;
    let mut posy = 20;
    loop {
        posx += 1;
        posy += 1;
        if posx > context.width() {
            posx = 20;
        }
        if posx > context.width() {
            posx = 20;
        }
    }
}
