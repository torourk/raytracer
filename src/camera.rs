use crate::vector::Vector3;

/// Represents state of the Viewport Camera
#[derive(Debug, Copy, Clone)]
pub struct Camera {
    position: Vector3,
    up: Vector3,
    fov: f32,
    pitch: f32,
    yaw: f32,
}

impl Camera {
    /// Creates a new camera with given parameters
    pub fn new(position: Vector3, up: Vector3, fov: f32, pitch: f32, yaw: f32) -> Camera {
        Camera {
            position,
            up,
            fov,
            pitch,
            yaw,
        }
    }

    /// Calculates the direction vector of the Camera
    pub fn direction(&self) -> Vector3 {
        let x = self.yaw.to_radians().sin() * self.pitch.to_radians().cos();
        let y = self.pitch.to_radians().sin();
        let z = self.yaw.to_radians().cos() * self.pitch.to_radians().cos();
        let out = Vector3::new(x, y, z);
        out.normalize()
    }

    /// Gets the position of the Camera
    pub fn position(&self) -> Vector3 {
        self.position
    }

    /// Gets the position of the Camera
    pub fn up(&self) -> Vector3 {
        self.up
    }

    /// Gets the position of the Camera
    pub fn fov(&self) -> f32 {
        self.fov
    }
}
