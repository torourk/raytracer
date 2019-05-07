use std::f32;
use std::ops::{Add, Mul, Neg, Sub};

/// Vector3 struct holds three 32-bit floats
#[derive(Debug, Copy, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    /// Returns Vector3(0.0, 0.0, 0.0)
    pub fn origin() -> Vector3 {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// Creates a Vector3 with 3 given coordinates
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }

    /// Returns Vector3(n, n, n)
    pub fn new_scalar(n: f32) -> Vector3 {
        Vector3 { x: n, y: n, z: n }
    }

    /// Takes the dot product between two vectors
    pub fn dot(self, other: Vector3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Gets the length of the given vector
    pub fn len(self) -> f32 {
        self.dot(self).sqrt()
    }

    /// Takes the crossproduct of two vectors
    pub fn cross(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// Normalizes a vector
    pub fn normalize(self) -> Vector3 {
        self * (1.0f32 / self.len())
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    /// Adds together two vectors
    fn add(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    /// Subtracts two vectors
    fn sub(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul for Vector3 {
    type Output = Vector3;

    /// Multiplies two vectors
    fn mul(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;

    /// Multiplies a vector with a scalar
    fn mul(self, rhs: f32) -> Vector3 {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    /// Negates a vector
    fn neg(self) -> Vector3 {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
