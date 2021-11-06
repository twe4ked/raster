// https://github.com/ssloy/tinyrenderer/wiki/Lesson-0:-getting-started

mod image;
mod obj;
mod ppm;
mod vec2;
mod vec3;

use image::Image;
use vec2::Vec2;
use vec3::Vec3;

use std::io::{self, BufWriter, Read, Write};

const WHITE: Vec3 = Vec3::new(255.0, 255.0, 255.0);

fn line(mut v0: Vec2, mut v1: Vec2, color: Vec3, image: &mut Image) {
    let mut transposed = false;

    if (v0.x as isize - v1.x as isize).abs() < (v0.y as isize - v1.y as isize).abs() {
        // If the lines is steep we transpose the image
        std::mem::swap(&mut v0.x, &mut v0.y);
        std::mem::swap(&mut v1.x, &mut v1.y);
        transposed = true;
    }

    if v0.x > v1.x {
        // Make it left−to−right
        std::mem::swap(&mut v0.x, &mut v1.x);
        std::mem::swap(&mut v0.y, &mut v1.y);
    }

    let dx = v1.x as isize - v0.x as isize;
    let dy = v1.y as isize - v0.y as isize;

    let derror = dy.abs() * 2;
    let mut error = 0;

    let mut y = v0.y as isize;
    for x in (v0.x as usize)..=(v1.x as usize) {
        if transposed {
            image.set(y as usize, x, color);
        } else {
            image.set(x, y as usize, color);
        };

        error += derror;
        if error > dx {
            y += if v1.y > v0.y { 1 } else { -1 };
            error -= dx * 2;
        }
    }
}

fn barycentric(pts: [Vec2; 3], p: Vec2) -> Vec3 {
    let cross = |a: Vec3, b| a.cross(&b);
    let u = cross(
        Vec3::new(pts[2].x - pts[0].x, pts[1].x - pts[0].x, pts[0].x - p.x),
        Vec3::new(pts[2].y - pts[0].y, pts[1].y - pts[0].y, pts[0].y - p.y),
    );

    // Return a negative result to discard the triangle
    if u.z.abs() < 1.0 {
        return Vec3::new(-1.0, 1.0, 1.0);
    }

    Vec3::new(1.0 - (u.x + u.y) / u.z, u.y / u.z, u.x / u.z)
}

fn triangle(pts: [Vec2; 3], color: Vec3, image: &mut Image) {
    let cols = image.cols as f32 - 1.0;
    let rows = image.rows as f32 - 1.0;

    let clamp = Vec2::new(cols, rows);
    let mut bboxmin = Vec2::new(cols, rows);
    let mut bboxmax = Vec2::new(0.0, 0.0);
    for point in &pts {
        bboxmin.x = 0.0_f32.max(bboxmin.x.min(point.x));
        bboxmax.x = clamp.x.min(bboxmax.x.max(point.x));
        bboxmin.y = 0.0_f32.max(bboxmin.y.min(point.y));
        bboxmax.y = clamp.y.min(bboxmax.y.max(point.y));
    }

    for x in (bboxmin.x as usize)..=(bboxmax.x as usize) {
        for y in (bboxmin.y as usize)..=(bboxmax.y as usize) {
            let p = Vec2::new(x as f32, y as f32);
            let bc_screen = barycentric(pts, p);
            if !(bc_screen.x < 0.0 || bc_screen.y < 0.0 || bc_screen.z < 0.0) {
                image.set(p.x as usize, p.y as usize, color);
            }
        }
    }
}

fn main() {
    let obj = read_stdin();
    let model = obj::parse(obj);

    let mut image = Image::new(1024, 1024);

    let padding = 25.0;
    let cols = image.cols as f32 - padding * 2.0;
    let rows = image.rows as f32 - padding * 2.0;

    let light_dir = Vec3::new(0.0, 0.0, -1.0);

    for face in &model.faces {
        let mut screen_coords = [Vec2::default(); 3];
        let mut world_coords = [Vec3::default(); 3];

        for j in 0..3 {
            let v = model.vertices[face[j]];

            screen_coords[j] = Vec2::new(
                ((v.x + 1.0) * cols / 2.0) + padding, // x
                ((v.y + 1.0) * rows / 2.0) + padding, // y
            );
            world_coords[j] = v;
        }

        let n = (world_coords[2] - world_coords[0]) ^ (world_coords[1] - world_coords[0]);
        let n = n.normalized();
        let intensity = n.dot(&light_dir);

        if intensity > 0.0 {
            let color = Vec3::new(intensity * 255.0, intensity * 255.0, intensity * 255.0);

            triangle(screen_coords, color, &mut image);
        }
    }

    for face in model.faces {
        for j in 0..3 {
            let v0 = model.vertices[face[j]];
            let v1 = model.vertices[face[(j + 1) % 3]];

            let x0 = (v0.x + 1.0) * cols / 2.0;
            let y0 = (v0.y + 1.0) * rows / 2.0;
            let x1 = (v1.x + 1.0) * cols / 2.0;
            let y1 = (v1.y + 1.0) * rows / 2.0;

            line(
                Vec2::new(x0 + padding, y0 + padding),
                Vec2::new(x1 + padding, y1 + padding),
                WHITE,
                &mut image,
            );
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

fn read_stdin() -> String {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();
    String::from_utf8(input).expect("invalid input")
}
