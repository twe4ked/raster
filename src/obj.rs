// https://en.wikipedia.org/wiki/Wavefront_.obj_file

use crate::vec2::Vec2;
use crate::vec3::Vec3;

#[derive(Default)]
pub struct Face {
    pub vertex_index: usize,
    pub texture_index: usize,
}

#[derive(Default)]
pub struct Model {
    pub vertices: Vec<Vec3>,
    pub faces: Vec<[Face; 3]>,
    pub texture_vertices: Vec<Vec2>,
}

pub fn parse(input: String) -> Model {
    let mut model = Model::default();

    for line in input.lines() {
        if line.starts_with("v ") {
            // Example:
            //      v 0.608654 -0.568839 -0.416318
            let mut parts = line.split(' ');

            parts.next().unwrap(); // v

            let x: f32 = parts.next().unwrap().parse().unwrap();
            let y: f32 = parts.next().unwrap().parse().unwrap();
            let z: f32 = parts.next().unwrap().parse().unwrap();

            model.vertices.push(Vec3::new(x, y, z));
        } else if line.starts_with("f ") {
            // Example:
            //      f 24/1/24 25/2/25 26/3/26
            let mut parts = line.split(' ');

            parts.next().unwrap(); // f

            let mut f1 = parts.next().unwrap().split('/');
            let f1 = Face {
                vertex_index: f1.next().unwrap().parse::<usize>().unwrap() - 1,
                texture_index: f1.next().unwrap().parse::<usize>().unwrap() - 1,
            };

            let mut f2 = parts.next().unwrap().split('/');
            let f2 = Face {
                vertex_index: f2.next().unwrap().parse::<usize>().unwrap() - 1,
                texture_index: f2.next().unwrap().parse::<usize>().unwrap() - 1,
            };

            let mut f3 = parts.next().unwrap().split('/');
            let f3 = Face {
                vertex_index: f3.next().unwrap().parse::<usize>().unwrap() - 1,
                texture_index: f3.next().unwrap().parse::<usize>().unwrap() - 1,
            };

            // Convert to 0-indexed
            model.faces.push([f1, f2, f3]);
        } else if line.starts_with("vt ") {
            // Example:
            //      vt  0.532 0.923 0.000
            let mut parts = line.split_ascii_whitespace();

            parts.next().unwrap(); // vt

            let u: f32 = parts.next().unwrap().parse().unwrap();
            let v: f32 = parts.next().unwrap().parse().unwrap();

            model.texture_vertices.push(Vec2::new(u, v));
        }
    }

    model
}
