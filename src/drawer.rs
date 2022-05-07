use std::f64;
use wasm_bindgen::JsCast;

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

    let mut posx = 75.0;
    let mut posy = 75.0;
    // loop {
    //     posx += 1.0;
    //     posy += 1.0;

    //     context.begin_path();
    //     context
    //         .arc(posx, posy, 50.0, 0.0, f64::consts::PI * 2.0)
    //         .unwrap();
    //     context.stroke();

    //     if posx > canvas.width().into() {
    //         posx = 75.0;
    //     }
    //     if posy > canvas.height().into() {
    //         posy = 75.0;
    //     }
    // }
}
