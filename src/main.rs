// https://github.com/ssloy/tinyrenderer/wiki/Lesson-0:-getting-started

mod obj;
mod ppm;

use std::io::{self, BufWriter, Read, Write};

#[derive(Copy, Clone, Default, Debug)]
pub struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

use std::ops::{Add, BitXor, Div, Mul, Sub};

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn normalized(&self) -> Self {
        *self / self.length()
    }

    fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl BitXor for Vec3 {
    type Output = Self;

    fn bitxor(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        (1.0 / rhs) * self
    }
}

const WHITE: Vec3 = Vec3 {
    x: 255.0,
    y: 255.0,
    z: 255.0,
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
    for x in (v0.x as usize)..=(v1.x as usize) {
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

fn triangle(mut t0: Vec2, mut t1: Vec2, mut t2: Vec2, color: Vec3, image: &mut [Vec<Vec3>]) {
    if t0.y > t1.y {
        std::mem::swap(&mut t0, &mut t1);
    }
    if t0.y > t2.y {
        std::mem::swap(&mut t0, &mut t2);
    }
    if t1.y > t2.y {
        std::mem::swap(&mut t1, &mut t2);
    }

    let total_height = t2.y - t0.y;

    for i in 0..(total_height as usize) {
        let second_half = i as f32 > t1.y - t0.y || t1.y == t0.y;

        let segment_height = if second_half {
            t2.y - t1.y
        } else {
            t1.y - t0.y
        };

        let alpha = i as f32 / total_height;

        let beta = if second_half {
            i as f32 - (t1.y - t0.y)
        } else {
            i as f32
        } / segment_height;

        let mut ax = (t0 + (t2 - t0) * alpha).x as usize;

        let mut bx = if second_half {
            t1 + (t2 - t1) * beta
        } else {
            t0 + (t1 - t0) * beta
        }
        .x as usize;

        if ax > bx {
            std::mem::swap(&mut ax, &mut bx);
        }

        for x in ax..=bx {
            image[t0.y as usize + i][x] = color;
        }
    }
}

fn main() {
    let obj = read_stdin();
    let model = obj::parse(obj);

    let mut image = vec![vec![Vec3::default(); COLS]; ROWS];

    let padding = 25.0;
    let cols = COLS as f32 - padding * 2.0;
    let rows = ROWS as f32 - padding * 2.0;

    let light_dir = Vec3::new(0.0, 0.0, -1.0);

    for face in model.faces.clone() {
        let mut screen_coords = [Vec2::default(); 3];
        let mut world_coords = [(0.0f32, 0.0, 0.0); 3];

        for j in 0..3 {
            let v = model.vertices[face[j] - 1];

            screen_coords[j] = Vec2::new(
                ((v.0 + 1.0) * cols / 2.0) + padding, // x
                ((v.1 + 1.0) * rows / 2.0) + padding, // y
            );
            world_coords[j] = v;
        }

        let wc0 = Vec3::new(world_coords[0].0, world_coords[0].1, world_coords[0].2);
        let wc1 = Vec3::new(world_coords[1].0, world_coords[1].1, world_coords[1].2);
        let wc2 = Vec3::new(world_coords[2].0, world_coords[2].1, world_coords[2].2);

        let n = (wc2 - wc0) ^ (wc1 - wc0);
        let n = n.normalized();
        let intensity = n.dot(&light_dir);

        if intensity > 0.0 {
            let color = Vec3::new(intensity * 255.0, intensity * 255.0, intensity * 255.0);

            triangle(
                screen_coords[0],
                screen_coords[1],
                screen_coords[2],
                color,
                &mut image,
            );
        }
    }

    for face in model.faces {
        for j in 0..3 {
            let v0 = model.vertices[face[j] - 1];
            let v1 = model.vertices[face[(j + 1) % 3] - 1];

            let x0 = (v0.0 + 1.0) * cols / 2.0;
            let y0 = (v0.1 + 1.0) * rows / 2.0;
            let x1 = (v1.0 + 1.0) * cols / 2.0;
            let y1 = (v1.1 + 1.0) * rows / 2.0;

            line(
                Vec2::new(x0 + padding, y0 + padding),
                Vec2::new(x1 + padding, y1 + padding),
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