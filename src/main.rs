// https://github.com/ssloy/tinyrenderer/wiki/Lesson-0:-getting-started

mod obj;
mod ppm;

use std::io::{self, BufWriter, Read, Write};

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

fn line(
    mut x0: usize,
    mut y0: usize,
    mut x1: usize,
    mut y1: usize,
    color: Vec3,
    image: &mut [Vec3],
) {
    let mut transposed = false;

    if (x0 as isize - x1 as isize).abs() < (y0 as isize - y1 as isize).abs() {
        // If the lines is steep we transpose the image
        std::mem::swap(&mut x0, &mut y0);
        std::mem::swap(&mut x1, &mut y1);
        transposed = true;
    }

    if x0 > x1 {
        // Make it left−to−right
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }

    let dx = x1 as isize - x0 as isize;
    let dy = y1 as isize - y0 as isize;

    let derror = dy.abs() * 2;
    let mut error = 0;

    let mut y = y0 as isize;
    for x in x0..=x1 {
        let idx = if transposed {
            idx(y as usize, x)
        } else {
            idx(x, y as usize)
        };
        image[idx] = color;

        error += derror;
        if error > dx {
            y += if y1 > y0 { 1 } else { -1 };
            error -= dx * 2;
        }
    }
}

fn main() {
    let obj = read_stdin();
    let model = obj::parse(obj);

    let mut image = vec![Vec3::default(); ROWS * COLS];

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
                x0 as usize + padding,
                y0 as usize + padding,
                x1 as usize + padding,
                y1 as usize + padding,
                WHITE,
                &mut image,
            );
        }
    }

    let mut stdout = BufWriter::new(io::stdout());
    ppm::write_header(&mut stdout, COLS, ROWS).unwrap();
    for color in image.iter().rev() {
        ppm::write_color(&mut stdout, color).unwrap();
    }
    stdout.flush().unwrap();
}

fn idx(x: usize, y: usize) -> usize {
    y * COLS + x
}

fn read_stdin() -> String {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();
    String::from_utf8(input).expect("invalid input")
}
