// https://github.com/ssloy/tinyrenderer/wiki/Lesson-0:-getting-started

mod geometry;
mod image;
mod obj;
mod ppm;
mod tga;
mod vec;

use image::Image;
use tga::TgaImage;
use vec::{Vec2, Vec3};

use std::env;
use std::fs::File;
use std::io::{self, BufWriter, Read, Write};

const WHITE: Vec3 = Vec3::new(255.0, 255.0, 255.0);

fn main() {
    let (model_filename, diffuse_filename, draw_lines) = {
        let mut model_filename = None;
        let mut diffuse_filename = None;
        let mut lines = false;
        for argument in env::args() {
            if let Some(a) = argument.strip_prefix("--model=") {
                model_filename = Some(a.to_string());
            }
            if let Some(a) = argument.strip_prefix("--diffuse=") {
                diffuse_filename = Some(a.to_string());
            }
            if argument == "--lines" {
                lines = true;
            }
        }
        (
            model_filename.expect("missing --model=MODEL.obj"),
            diffuse_filename.expect("missing --diffuse=DIFFUSE.tga"),
            lines,
        )
    };

    let model = {
        let mut data = Vec::new();
        let mut f = File::open(model_filename).unwrap();
        f.read_to_end(&mut data).unwrap();
        let obj = String::from_utf8(data).expect("invalid input");
        obj::parse(obj)
    };

    let texture = {
        let mut data = Vec::new();
        let mut f = File::open(diffuse_filename).unwrap();
        f.read_to_end(&mut data).unwrap();
        tga::parse(&data)
    };

    let mut image = Image::new(1024, 1024);
    let mut zbuffer = vec![isize::MIN; image.cols * image.rows];

    let padding = 25.0;
    let cols = image.cols as f32 - padding * 2.0;
    let rows = image.rows as f32 - padding * 2.0;

    let to_screen_coords =
        |v: Vec3| ((Vec2::from(v) + 1.0) * Vec2::new(cols, rows) / 2.0) + padding;

    let light_dir = Vec3::new(0.0, 0.0, -1.0);

    for face in &model.faces {
        let mut screen_coords = [Vec2::default(); 3];
        let mut world_coords = [Vec3::default(); 3];
        let mut texture_coords = [Vec2::default(); 3];

        for j in 0..3 {
            let v = model.vertices[face[j].vertex_index];

            screen_coords[j] = to_screen_coords(v);
            world_coords[j] = v;
            texture_coords[j] = model.texture_vertices[face[j].texture_index]
        }

        let n = (world_coords[2] - world_coords[0]) ^ (world_coords[1] - world_coords[0]);
        let n = n.normalized();
        let intensity = n.dot(&light_dir);

        if intensity > 0.0 {
            geometry::triangle(
                screen_coords,
                texture_coords,
                &mut zbuffer,
                &texture,
                intensity,
                &mut image,
            );
        }
    }

    if draw_lines {
        for face in model.faces {
            for j in 0..3 {
                let v0 = model.vertices[face[j].vertex_index];
                let v1 = model.vertices[face[(j + 1) % 3].vertex_index];

                geometry::line(
                    to_screen_coords(v0),
                    to_screen_coords(v1),
                    WHITE,
                    &mut image,
                );
            }
        }
    }

    let mut stdout = BufWriter::new(io::stdout());
    ppm::write_header(&mut stdout, image.cols, image.rows).unwrap();
    for row in image.data.iter().rev() {
        for color in row {
            ppm::write_color(&mut stdout, color).unwrap();
        }
    }
    stdout.flush().unwrap();
}
