use crate::vector::Vector3;

/// Reflection Refraction Index allows storing reflection/refraction data
/// with three options: Diffuse, Reflective, and Reflective/Refractive.
pub type ReflectionRefractionIndex = Option<(f32, Option<f32>)>;

/// Describes a ray intersection of the surface of an intersectable
#[derive(Debug)]
pub struct RayHit {
    position: Vector3,
    normal: Vector3,
    distance: f32,
    color: Vector3,
    reflect_and_refract: ReflectionRefractionIndex,
}

impl RayHit {
    /// Creates a new RayHit with the given hit data
    pub fn new(
        position: Vector3,
        normal: Vector3,
        distance: f32,
        color: Vector3,
        reflect_and_refract: ReflectionRefractionIndex,
    ) -> RayHit {
        RayHit {
            position,
            normal,
            distance,
            color,
            reflect_and_refract,
        }
    }

    /// Gets the intersect position of the hit
    pub fn position(&self) -> Vector3 {
        self.position
    }

    /// Gets the normal of the intersection on the surface
    pub fn normal(&self) -> Vector3 {
        self.normal
    }

    /// Gets the distance between the ray origin and intersection point
    pub fn distance(&self) -> f32 {
        self.distance
    }

    /// Gets the color of the point that was hit
    pub fn color(&self) -> Vector3 {
        self.color
    }

    /// Gets the optional reflection and refraction index
    pub fn reflection_and_refraction_index(&self) -> ReflectionRefractionIndex {
        self.reflect_and_refract
    }
}
