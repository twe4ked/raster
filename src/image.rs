use crate::vec3::Vec3;

pub struct Image {
    pub data: Vec<Vec<Vec3>>,
    pub cols: usize,
    pub rows: usize,
}

impl Image {
    pub fn new(cols: usize, rows: usize) -> Self {
        let data = vec![vec![Vec3::default(); cols]; rows];
        Self { data, cols, rows }
    }

    pub fn set(&mut self, x: usize, y: usize, color: Vec3) {
        self.data[y][x] = color;
    }
}
