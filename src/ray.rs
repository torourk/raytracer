use crate::vector::Vector3;

/// Ray comprised of origin and direction vectors
#[derive(Debug, Copy, Clone)]
pub struct Ray {
    origin: Vector3,
    direction: Vector3,
}

impl Ray {
    /// Creates a ray given an origin and direction
    pub fn new(origin: Vector3, direction: Vector3) -> Ray {
        Ray { origin, direction }
    }

    /// Gets the origin of the ray
    pub fn origin(&self) -> Vector3 {
        self.origin
    }

    /// Gets the direction of the ray
    pub fn direction(&self) -> Vector3 {
        self.direction
    }
}
