use std::collections::VecDeque;
use std::f64;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

use crate::camera::Camera;
use crate::transforms::Mat3D;
//use crate::tri2D::Tri2D;
use crate::tri3D::Tri3D;
use crate::vec3D::Vec3D;
#[wasm_bindgen]
pub struct Scene {
    canvas: web_sys::HtmlCanvasElement, // width and height, framebuffer
    ctx: web_sys::CanvasRenderingContext2d,
    // list of meshs || list of triangles
    tris: Vec<Tri3D>,
    camera: Camera,           // Word view to camera view
    projection_matrix: Mat3D, // 3D to 2D
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
        let screen_width = canvas.width() as f64;
        let screen_height = canvas.width() as f64;
        Scene {
            canvas,
            ctx,
            tris: Vec::from([
                Tri3D::from_points(0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0),
                Tri3D::from_points(0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0),
                Tri3D::from_points(1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0),
                Tri3D::from_points(1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0),
                Tri3D::from_points(1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0),
                Tri3D::from_points(1.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0),
                // Tri2D::new(
                //     Vec2D::new(0.9, 0.9),
                //     Vec2D::new(0.1, 0.1),
                //     Vec2D::new(0.5, 0.5),
                // ),
                // Tri2D::new(
                //     Vec2D::new(0.8, 0.8),
                //     Vec2D::new(0.7, 0.8),
                //     Vec2D::new(0.7, 0.7),
                // ),
                // Tri2D::new(
                //     Vec2D::new(0.4, 0.4),
                //     Vec2D::new(0.4, 0.7),
                //     Vec2D::new(0.7, 0.7),
                // ),
                // Tri2D::new(
                //     Vec2D::new(-0.9, -0.9),
                //     Vec2D::new(0.2, 0.7),
                //     Vec2D::new(-0.3, -0.4),
                // ),
            ]),
            camera: Camera::new(),
            projection_matrix: Mat3D::projection(90.0, screen_height / screen_width, 0.1, 1000.0),
        }
    }

    fn draw_hollow(tri: &Tri3D, ctx: &mut web_sys::CanvasRenderingContext2d) {
        ctx.begin_path();
        ctx.move_to(tri[0][0], tri[0][1]);
        ctx.line_to(tri[1][0], tri[1][1]);
        ctx.line_to(tri[2][0], tri[2][1]);
        ctx.close_path();
        ctx.stroke();
    }

    fn draw_tri(tri: &Tri3D, ctx: &mut web_sys::CanvasRenderingContext2d) {
        ctx.begin_path();
        ctx.move_to(tri[0][0], tri[0][1]);
        ctx.line_to(tri[1][0], tri[1][1]);
        ctx.line_to(tri[2][0], tri[2][1]);
        ctx.set_fill_style(&"rgb(200,200,200)".into());
        ctx.fill();
    }

    fn draw_from_vec(tris: &Vec<Tri3D>, ctx: &mut web_sys::CanvasRenderingContext2d) {
        for tri in tris {
            Scene::draw_tri(tri, ctx);
            Scene::draw_hollow(tri, ctx);
        }
    }

    pub fn tick(&mut self, time: f64) {
        self.ctx.clear_rect(
            0.into(),
            0.into(),
            self.canvas.width().into(),
            self.canvas.height().into(),
        );

        let mut tris: Vec<Tri3D> = self.tris.clone();
        Scene::apply_transforms(&mut tris, time);
        Scene::to_view(&mut tris, &mut self.camera);

        // clip near plane
        Scene::clip_tris(
            &mut tris,
            Vec3D::new(0.0, 0.0, 0.1),
            Vec3D::new(0.0, 0.0, 1.0),
        );

        Scene::project(&mut tris, &self.projection_matrix);
        Scene::to_ndc(&mut tris);
        Scene::ndc_to_screen(&mut tris, self.canvas.width(), self.canvas.height());
        // sort via z coordinate
        tris.sort();

        // clip each side of the screen
        Scene::clip_tris(
            &mut tris,
            Vec3D::new(0.0, 0.0, 0.0),
            Vec3D::new(0.0, 1.0, 0.0),
        );
        Scene::clip_tris(
            &mut tris,
            Vec3D::new(0.0, (self.canvas.height() - 1) as f64, 0.0),
            Vec3D::new(0.0, -1.0, 0.0),
        );
        Scene::clip_tris(
            &mut tris,
            Vec3D::new(0.0, 0.0, 0.0),
            Vec3D::new(1.0, 0.0, 0.0),
        );
        Scene::clip_tris(
            &mut tris,
            Vec3D::new((self.canvas.width() - 1) as f64, 0.0, 0.0),
            Vec3D::new(-1.0, 0.0, 0.0),
        );

        Scene::draw_from_vec(&tris, &mut self.ctx);
    }

    fn apply_transforms(tris: &mut Vec<Tri3D>, time: f64) {
        let rotating = &mut tris[3];

        rotating[0] = rotating[0].mul(&Mat3D::rot_x(time));
        rotating[1] = rotating[1].mul(&Mat3D::rot_x(time));
        rotating[2] = rotating[2].mul(&Mat3D::rot_x(time));
    }

    fn to_view(tris: &mut Vec<Tri3D>, camera: &mut Camera) {
        camera.compute_view();

        for i in 0..tris.len() {
            tris[i] = camera.to_view(tris[i]);
        }
    }

    fn clip_tris(tris: &mut Vec<Tri3D>, plane_pos: Vec3D, plane_normal: Vec3D) {
        let mut res = Vec::new();
        let plane_n = plane_normal.normalized();
        for tri in tris.iter() {
            let (nb_tris, tri1, tri2) = Tri3D::clip(&tri, &plane_pos, &plane_n);
            match nb_tris {
                0 => continue,
                1 => res.push(tri1.unwrap()),
                2 => {
                    res.push(tri1.unwrap());
                    res.push(tri2.unwrap())
                }
                _ => panic!("nb of tris illegal: {}", nb_tris),
            }
        }

        *tris = res;
    }

    fn project(tris: &mut Vec<Tri3D>, projection_matrix: &Mat3D) {
        for i in 0..tris.len() {
            tris[i] = tris[i].mul(projection_matrix);
        }
    }

    fn to_ndc(tris: &mut Vec<Tri3D>) {
        for tri in tris {
            for vertex in [tri[0], tri[1], tri[2]].iter_mut() {
                vertex.x = -1.0 * (vertex.x / vertex.w);
                vertex.y = -1.0 * (vertex.x / vertex.w);
                vertex.z = -1.0 * (vertex.x / vertex.w);
            }
        }
    }

    // take NDC coordinates [(-1,-1), (1,1)] to screen [(0, 0), (width, height)]
    // x = w(x + 1) / 2
    // y = h(y + 1) / 2
    // z = z
    fn ndc_to_screen(tris: &mut Vec<Tri3D>, width: u32, height: u32) {
        let w = width as f64;
        let h = height as f64;
        for tri in tris {
            tri[0].x = w * (tri[0].x + 1.0) * 0.5;
            tri[0].y = h * (tri[0].y + 1.0) * 0.5;

            tri[1].x = w * (tri[1].x + 1.0) * 0.5;
            tri[1].y = h * (tri[1].y + 1.0) * 0.5;

            tri[2].x = w * (tri[2].x + 1.0) * 0.5;
            tri[2].y = h * (tri[2].y + 1.0) * 0.5;
        }
    }
}
