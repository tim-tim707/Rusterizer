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
                Tri2D {
                    a: Vec2D { x: 1.0, y: 1.0 },
                    b: Vec2D { x: 0.1, y: 0.1 },
                    c: Vec2D { x: 0.3, y: 0.3 },
                },
                Tri2D {
                    a: Vec2D { x: 0.4, y: 0.4 },
                    b: Vec2D { x: 0.4, y: 0.7 },
                    c: Vec2D { x: 0.7, y: 0.7 },
                },
                Tri2D {
                    a: Vec2D { x: -0.9, y: -0.9 },
                    b: Vec2D { x: 0.2, y: 0.7 },
                    c: Vec2D { x: -0.3, y: -0.4 },
                },
            ]),
        }
    }

    fn draw_hollow(&self, tri: &Tri2D) {
        self.ctx.begin_path();
        let Vec2D { x, y } = tri.a;
        self.ctx.move_to(x, y);
        let Vec2D { x, y } = tri.b;
        self.ctx.line_to(x, y);
        let Vec2D { x, y } = tri.c;
        self.ctx.line_to(x, y);
        self.ctx.close_path();
        self.ctx.stroke();
    }

    fn draw_tri(&self, tri: &Tri2D) {
        self.ctx.begin_path();
        let Vec2D { x, y } = tri.a;
        self.ctx.move_to(x, y);
        let Vec2D { x, y } = tri.b;
        self.ctx.line_to(x, y);
        let Vec2D { x, y } = tri.c;
        self.ctx.line_to(x, y);
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
        for tri in &self.tris {
            let tri_fit: Tri2D = Tri2D {
                a: Vec2D {
                    x: (tri.a.x + 1.0) * (self.canvas.width() / 2) as f64,
                    y: (tri.a.y + 1.0) * (self.canvas.height() / 2) as f64,
                },
                b: Vec2D {
                    x: (tri.b.x + 1.0) * (self.canvas.width() / 2) as f64,
                    y: (tri.b.y + 1.0) * (self.canvas.height() / 2) as f64,
                },
                c: Vec2D {
                    x: (tri.c.x + 1.0) * (self.canvas.width() / 2) as f64,
                    y: (tri.c.y + 1.0) * (self.canvas.height() / 2) as f64,
                },
            };
            res.push(tri_fit);
        }
        res
    }
}
