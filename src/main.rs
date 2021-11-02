// https://github.com/ssloy/tinyrenderer/wiki/Lesson-0:-getting-started

mod obj;
mod ppm;

use std::io::{self, BufWriter, Read, Write};

#[derive(Copy, Clone, Default)]
pub struct Vec2 {
    x: usize,
    y: usize,
}

impl Vec2 {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Copy, Clone, Default)]
pub struct Vec3 {
    x: u8,
    y: u8,
    z: u8,
}

const WHITE: Vec3 = Vec3 {
    x: 255,
    y: 255,
    z: 255,
};

const COLS: usize = 1024;
const ROWS: usize = 1024;

fn line(mut v0: Vec2, mut v1: Vec2, color: Vec3, image: &mut [Vec<Vec3>]) {
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
    for x in v0.x..=v1.x {
        if transposed {
            image[x][y as usize] = color;
        } else {
            image[y as usize][x] = color;
        };

        error += derror;
        if error > dx {
            y += if v1.y > v0.y { 1 } else { -1 };
            error -= dx * 2;
        }
    }
}

fn main() {
    let obj = read_stdin();
    let model = obj::parse(obj);

    let mut image = vec![vec![Vec3::default(); COLS]; ROWS];

    let padding = 25;
    let cols = COLS - padding * 2;
    let rows = ROWS - padding * 2;

    for face in model.faces {
        for j in 0..3 {
            let v0 = model.vertices[face[j] - 1];
            let v1 = model.vertices[face[(j + 1) % 3] - 1];

            let x0 = (v0.0 + 1.0) * cols as f32 / 2.0;
            let y0 = (v0.1 + 1.0) * rows as f32 / 2.0;
            let x1 = (v1.0 + 1.0) * cols as f32 / 2.0;
            let y1 = (v1.1 + 1.0) * rows as f32 / 2.0;

            line(
                Vec2::new(x0 as usize + padding, y0 as usize + padding),
                Vec2::new(x1 as usize + padding, y1 as usize + padding),
                WHITE,
                &mut image,
            );
        }
    }

    let mut stdout = BufWriter::new(io::stdout());
    ppm::write_header(&mut stdout, COLS, ROWS).unwrap();
    for row in image.iter().rev() {
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
