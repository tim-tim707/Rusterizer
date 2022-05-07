use std::f64;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::tri2D::Tri2D;
use crate::vec2D::Vec2D;

#[wasm_bindgen]
pub struct Scene {
    canvas: web_sys::HtmlCanvasElement,
    ctx: web_sys::CanvasRenderingContext2d,
    tris: Vec<Tri2D>,
}

#[wasm_bindgen]
impl Scene {
    pub fn new(canvas: web_sys::HtmlCanvasElement) -> Scene {
        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        ctx.set_image_smoothing_enabled(true);
        Scene {
            canvas,
            ctx,
            tris: Vec::from([
                Tri2D::new(
                    Vec2D::new(1.0, 1.0),
                    Vec2D::new(0.1, 0.1),
                    Vec2D::new(0.3, 0.3),
                ),
                Tri2D::new(
                    Vec2D::new(0.4, 0.4),
                    Vec2D::new(0.4, 0.7),
                    Vec2D::new(0.7, 0.7),
                ),
                Tri2D::new(
                    Vec2D::new(-0.9, -0.9),
                    Vec2D::new(0.2, 0.7),
                    Vec2D::new(-0.3, -0.4),
                ),
            ]),
        }
    }

    fn draw_hollow(&self, tri: &Tri2D) {
        self.ctx.begin_path();
        self.ctx.move_to(tri[0][0], tri[0][1]);
        self.ctx.line_to(tri[1][0], tri[1][1]);
        self.ctx.line_to(tri[2][0], tri[2][1]);
        self.ctx.close_path();
        self.ctx.stroke();
    }

    fn draw_tri(&self, tri: &Tri2D) {
        self.ctx.begin_path();
        self.ctx.move_to(tri[0][0], tri[0][1]);
        self.ctx.line_to(tri[1][0], tri[1][1]);
        self.ctx.line_to(tri[2][0], tri[2][1]);
        self.ctx.set_fill_style(&"rgb(200,200,200)".into());
        self.ctx.fill();
    }

    pub fn draw(&self) {
        for tri in &self.tris {
            self.draw_tri(tri);
            self.draw_hollow(tri);
        }
    }

    fn draw_from_vec(&self, tris: Vec<Tri2D>) {
        for tri in tris {
            self.draw_tri(&tri);
            self.draw_hollow(&tri);
        }
    }

    pub fn tick(&mut self) {
        self.ctx.clear_rect(
            0.into(),
            0.into(),
            self.canvas.width().into(),
            self.canvas.height().into(),
        );
        self.tris[0].a.x += 0.02;
        self.draw_from_vec(self.ndc_to_screen());
    }

    // take NDC coordinates [(-1,-1), (1,1)] to screen [(0, 0), (width, height)]
    // note that this flip the screen on horizontal axis
    // x = w(x + 1) / 2
    // y = w(y + 1) / 2
    // z = w(x + 1) / 2
    fn ndc_to_screen(&self) -> Vec<Tri2D> {
        let mut res = Vec::new();
        let w = (self.canvas.width() / 2) as f64;
        let h = (self.canvas.height() / 2) as f64;
        for tri in &self.tris {
            let tri_fit: Tri2D = Tri2D::new(
                Vec2D::new((tri[0][0] + 1.0) * w, (tri[0][1] + 1.0) * h),
                Vec2D::new((tri[1][0] + 1.0) * w, (tri[1][1] + 1.0) * h),
                Vec2D::new((tri[2][0] + 1.0) * w, (tri[2][1] + 1.0) * h),
            );

            res.push(tri_fit);
        }
        res
    }
}
