use crate::{Image, TgaImage, Vec2, Vec3};

pub fn line(mut v0: Vec2, mut v1: Vec2, color: Vec3, image: &mut Image) {
    let mut transposed = false;

    if (v0[0] as isize - v1[0] as isize).abs() < (v0[1] as isize - v1[1] as isize).abs() {
        // If the lines is steep we transpose the image
        v0.swap(0, 1);
        v1.swap(0, 1);
        transposed = true;
    }

    if v0[0] > v1[0] {
        // Make it left−to−right
        std::mem::swap(&mut v0[0], &mut v1[0]);
        std::mem::swap(&mut v0[1], &mut v1[1]);
    }

    let dx = v1[0] as isize - v0[0] as isize;
    let dy = v1[1] as isize - v0[1] as isize;

    let derror = dy.abs() * 2;
    let mut error = 0;

    let mut y = v0[1] as isize;
    for x in (v0[0] as usize)..=(v1[0] as usize) {
        if transposed {
            image.set(y as usize, x, color);
        } else {
            image.set(x, y as usize, color);
        };

        error += derror;
        if error > dx {
            y += if v1[1] > v0[1] { 1 } else { -1 };
            error -= dx * 2;
        }
    }
}

fn barycentric(pts: [Vec2; 3], p: Vec2) -> Vec3 {
    let cross = |a: Vec3, b| a.cross(&b);
    let u = cross(
        Vec3::new(
            pts[2][0] - pts[0][0],
            pts[1][0] - pts[0][0],
            pts[0][0] - p[0],
        ),
        Vec3::new(
            pts[2][1] - pts[0][1],
            pts[1][1] - pts[0][1],
            pts[0][1] - p[1],
        ),
    );

    // Return a negative result to discard the triangle
    if u[2].abs() < 1.0 {
        return Vec3::new(-1.0, 1.0, 1.0);
    }

    Vec3::new(1.0 - (u[0] + u[1]) / u[2], u[1] / u[2], u[0] / u[2])
}

pub fn triangle(
    pts: [Vec2; 3],
    texture_coords: [Vec2; 3],
    zbuffer: &mut [isize],
    texture: &TgaImage,
    intensity: f32,
    image: &mut Image,
) {
    let cols = image.cols as f32 - 1.0;
    let rows = image.rows as f32 - 1.0;

    let clamp = Vec2::new(cols, rows);
    let mut bboxmin = Vec2::new(cols, rows);
    let mut bboxmax = Vec2::new(0.0, 0.0);
    for point in &pts {
        bboxmin[0] = 0.0_f32.max(bboxmin[0].min(point[0]));
        bboxmax[0] = clamp[0].min(bboxmax[0].max(point[0]));
        bboxmin[1] = 0.0_f32.max(bboxmin[1].min(point[1]));
        bboxmax[1] = clamp[1].min(bboxmax[1].max(point[1]));
    }

    // Find the bounding box of the texture
    let mut tbboxmin = Vec2::new(f32::MAX, f32::MAX);
    let mut tbboxmax = Vec2::new(0.0, 0.0);

    for point in &texture_coords {
        tbboxmin[0] = tbboxmin[0].min(point[0]);
        tbboxmax[0] = tbboxmax[0].max(point[0]);
        tbboxmin[1] = tbboxmin[1].min(point[1]);
        tbboxmax[1] = tbboxmax[1].max(point[1]);
    }

    let texture_width = texture.width as f32;
    let texture_height = texture.height as f32;

    for x in (bboxmin[0] as usize)..=(bboxmax[0] as usize) {
        for y in (bboxmin[1] as usize)..=(bboxmax[1] as usize) {
            let p = Vec2::new(x as f32, y as f32);
            let bc_screen = barycentric(pts, p);
            if !(bc_screen[0] < 0.0 || bc_screen[1] < 0.0 || bc_screen[2] < 0.0) {
                let z = 0;
                let zbuffer_i = p[0] as usize + p[1] as usize * image.cols;
                if zbuffer[zbuffer_i] < z as isize {
                    zbuffer[zbuffer_i] = z as isize;

                    let x_ratio = (p[0] - bboxmin[0]) / (bboxmax[0] - bboxmin[0]);
                    let y_ratio = (p[1] - bboxmin[1]) / (bboxmax[1] - bboxmin[1]);
                    let mut tx =
                        (tbboxmin[0] + (tbboxmax[0] - tbboxmin[0]) * x_ratio) * texture_width;
                    let mut ty =
                        (tbboxmin[1] + (tbboxmax[1] - tbboxmin[1]) * y_ratio) * texture_height;

                    // Flip the texture (the image origin bottom left)
                    tx = texture_width - tx;
                    ty = texture_height - ty;

                    // Get the color from the texture
                    let color = texture.pixels[&(tx as usize, ty as usize)];

                    image.set(p[0] as usize, p[1] as usize, color * intensity);
                }
            }
        }
    }
}
