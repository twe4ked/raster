use crate::{Image, TgaImage, Vec2, Vec3};

pub fn line(mut v0: Vec2, mut v1: Vec2, color: Vec3, image: &mut Image) {
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
        bboxmin.x = 0.0_f32.max(bboxmin.x.min(point.x));
        bboxmax.x = clamp.x.min(bboxmax.x.max(point.x));
        bboxmin.y = 0.0_f32.max(bboxmin.y.min(point.y));
        bboxmax.y = clamp.y.min(bboxmax.y.max(point.y));
    }

    // Find the bounding box of the texture
    let mut tbboxmin = Vec2::new(f32::MAX, f32::MAX);
    let mut tbboxmax = Vec2::new(0.0, 0.0);

    for point in &texture_coords {
        tbboxmin.x = tbboxmin.x.min(point.x);
        tbboxmax.x = tbboxmax.x.max(point.x);
        tbboxmin.y = tbboxmin.y.min(point.y);
        tbboxmax.y = tbboxmax.y.max(point.y);
    }

    let texture_width = texture.width as f32;
    let texture_height = texture.height as f32;

    for x in (bboxmin.x as usize)..=(bboxmax.x as usize) {
        for y in (bboxmin.y as usize)..=(bboxmax.y as usize) {
            let p = Vec2::new(x as f32, y as f32);
            let bc_screen = barycentric(pts, p);
            if !(bc_screen.x < 0.0 || bc_screen.y < 0.0 || bc_screen.z < 0.0) {
                let z = 0;
                let zbuffer_i = p.x as usize + p.y as usize * image.cols;
                if zbuffer[zbuffer_i] < z as isize {
                    zbuffer[zbuffer_i] = z as isize;

                    let x_ratio = (p.x - bboxmin.x) / (bboxmax.x - bboxmin.x);
                    let y_ratio = (p.y - bboxmin.y) / (bboxmax.y - bboxmin.y);
                    let mut tx = (tbboxmin.x + (tbboxmax.x - tbboxmin.x) * x_ratio) * texture_width;
                    let mut ty =
                        (tbboxmin.y + (tbboxmax.y - tbboxmin.y) * y_ratio) * texture_height;

                    // Flip the texture (the image origin bottom left)
                    tx = texture_width - tx;
                    ty = texture_height - ty;

                    // Get the color from the texture
                    let color = texture.pixels[&(tx as usize, ty as usize)];

                    image.set(p.x as usize, p.y as usize, color * intensity);
                }
            }
        }
    }
}
