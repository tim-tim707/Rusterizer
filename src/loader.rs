use std::io::{BufReader, Read};

use crate::{tri3D::Tri3D, vec3D::Vec3D};

fn read_float(buf_reader: &mut BufReader<&[u8]>) -> f64 {
    let mut float_buffer = [0u8; std::mem::size_of::<f32>()];
    let e = buf_reader.read_exact(&mut float_buffer);
    if e.is_err() {
        panic!("{:?}", e);
    }
    f32::from_le_bytes(float_buffer) as f64
}

pub fn load_teapot() -> Vec<Tri3D> {
    let teapot = include_bytes!("teapot.stl");
    let mut res = Vec::new();

    // header 80 bytes + nb_triangles u32: 4 bytes, little endian
    let mut buf_reader = BufReader::new(&teapot[..]);
    let mut header: [u8; 80] = [0; 80];
    if buf_reader.read_exact(&mut header).is_err() {
        return Vec::new();
    }
    let mut u32_buf = [0u8; std::mem::size_of::<u32>()];
    if buf_reader.read_exact(&mut u32_buf).is_err() {
        return Vec::new();
    }

    let nb_tris = u32::from_le_bytes(u32_buf);

    for _ in 0..nb_tris {
        // normal vector: 3 * f32 -- ignored
        // vertex 1: 3 * f32
        // vertex 2: 3 * f32
        // vertex 3: 3 * f32
        // attribute byte count: 2 bytes, everything little endian -- ignored
        let _normal_x = read_float(&mut buf_reader);
        let _normal_y = read_float(&mut buf_reader);
        let _normal_z = read_float(&mut buf_reader);
        res.push(Tri3D::new(
            Vec3D::new(
                read_float(&mut buf_reader),
                read_float(&mut buf_reader),
                read_float(&mut buf_reader),
            ),
            Vec3D::new(
                read_float(&mut buf_reader),
                read_float(&mut buf_reader),
                read_float(&mut buf_reader),
            ),
            Vec3D::new(
                read_float(&mut buf_reader),
                read_float(&mut buf_reader),
                read_float(&mut buf_reader),
            ),
        ));
        let mut attribute_byte_count = [0u8; std::mem::size_of::<u16>()];
        let e = buf_reader.read_exact(&mut attribute_byte_count);
        if e.is_err() {
            panic!("{:?}", e);
        }
    }
    res
}
