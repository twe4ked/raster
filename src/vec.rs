use std::ops::{Add, BitXor, Div, Index, IndexMut, Mul, Sub};

pub type Vec2 = Vec<2>;
pub type Vec3 = Vec<3>;

#[derive(Copy, Clone, Debug)]
pub struct Vec<const N: usize> {
    components: [f32; N],
}

impl<const N: usize> Default for Vec<N> {
    fn default() -> Self {
        Self {
            components: [0.0; N],
        }
    }
}

impl<const N: usize> Index<usize> for Vec<N> {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.components[index]
    }
}

impl<const N: usize> IndexMut<usize> for Vec<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.components[index]
    }
}

impl Vec<2> {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { components: [x, y] }
    }
}

impl Vec<3> {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            components: [x, y, z],
        }
    }

    pub fn cross(&self, v: &Self) -> Self {
        let u = self;
        Self::new(
            u[1] * v[2] - u[2] * v[1],
            u[2] * v[0] - u[0] * v[2],
            u[0] * v[1] - u[1] * v[0],
        )
    }
}

impl<const N: usize> Vec<N> {
    pub fn swap(&mut self, i: usize, j: usize) {
        self.components.swap(i, j);
    }

    fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    fn length_squared(&self) -> f32 {
        self.components.iter().map(|c| c * c).sum()
    }

    pub fn normalized(&self) -> Self {
        *self / self.length()
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self[0] * other[0] + self[1] * other[1] + self[2] * other[2]
    }
}

impl<const N: usize> Mul<f32> for Vec<N> {
    type Output = Vec<N>;

    fn mul(mut self, rhs: f32) -> Self {
        for c in self.components.iter_mut() {
            *c *= rhs;
        }
        self
    }
}

impl<const N: usize> Add<f32> for Vec<N> {
    type Output = Vec<N>;

    fn add(mut self, rhs: f32) -> Self {
        for c in self.components.iter_mut() {
            *c += rhs;
        }
        self
    }
}

impl<const N: usize> Mul<Vec<N>> for f32 {
    type Output = Vec<N>;

    fn mul(self, rhs: Vec<N>) -> Self::Output {
        rhs * self
    }
}

impl<const N: usize> Sub for Vec<N> {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self {
        for (c, d) in self.components.iter_mut().zip(other.components) {
            *c -= d;
        }
        self
    }
}

impl<const N: usize> Mul for Vec<N> {
    type Output = Self;

    fn mul(mut self, other: Self) -> Self {
        for (c, d) in self.components.iter_mut().zip(other.components) {
            *c *= d;
        }
        self
    }
}

impl BitXor for Vec<3> {
    type Output = Self;

    fn bitxor(self, other: Self) -> Self {
        Self::new(
            self[1] * other[2] - self[2] * other[1],
            self[2] * other[0] - self[0] * other[2],
            self[0] * other[1] - self[1] * other[0],
        )
    }
}

impl<const N: usize> Div<f32> for Vec<N> {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        (1.0 / rhs) * self
    }
}

impl From<Vec3> for Vec2 {
    fn from(other: Vec3) -> Self {
        Vec2 {
            components: [other.components[0], other.components[1]],
        }
    }
}
