// https://en.wikipedia.org/wiki/Wavefront_.obj_file

use crate::vec3::Vec3;

pub struct Model {
    pub vertices: Vec<Vec3>,
    pub faces: Vec<[usize; 3]>,
}

pub fn parse(input: String) -> Model {
    let mut vertices = Vec::new();
    let mut faces = Vec::new();

    for line in input.lines() {
        if line.starts_with("v ") {
            // Example:
            //      v 0.608654 -0.568839 -0.416318
            let mut parts = line.split(' ');

            parts.next().unwrap(); // v

            let x: f32 = parts.next().unwrap().parse().unwrap();
            let y: f32 = parts.next().unwrap().parse().unwrap();
            let z: f32 = parts.next().unwrap().parse().unwrap();

            vertices.push(Vec3::new(x, y, z));
        } else if line.starts_with("f ") {
            // Example:
            //      f 24/1/24 25/2/25 26/3/26
            let mut parts = line.split(' ');

            parts.next().unwrap(); // f

            let f1 = parts.next().unwrap();
            let f1: usize = f1.split('/').next().unwrap().parse().unwrap();

            let f2 = parts.next().unwrap();
            let f2: usize = f2.split('/').next().unwrap().parse().unwrap();

            let f3 = parts.next().unwrap();
            let f3: usize = f3.split('/').next().unwrap().parse().unwrap();

            // Convert to 0-indexed
            faces.push([f1 - 1, f2 - 1, f3 - 1]);
        }
    }

    Model { vertices, faces }
}
