use crate::intersectable::Intersectable;
use crate::ray::Ray;
use crate::rayhit::{RayHit, ReflectionRefractionIndex};
use crate::vector::Vector3;

/// A sphere with a position, color, and radius
#[derive(Debug)]
pub struct Plane {
    origin: Vector3,
    normal: Vector3,
    color: Vector3,
    reflection_and_refraction: ReflectionRefractionIndex,
}

impl Plane {
    /// Creates a new sphere with given geometric data
    pub fn new(
        origin: Vector3,
        normal: Vector3,
        color: Vector3,
        reflection_and_refraction: ReflectionRefractionIndex,
    ) -> Plane {
        Plane {
            origin,
            normal,
            color,
            reflection_and_refraction,
        }
    }
}

impl Intersectable for Plane {
    /// Determines whether the given ray has intersected with the sphere
    /// and generates a RayHit
    fn intersect(&self, ray: Ray) -> Option<RayHit> {
        
        let denom = (-self.normal).dot(ray.direction());
        if denom > 0.001 {
            let ray_to_origin = self.origin - ray.origin();
            let t = ray_to_origin.dot(-self.normal) / denom;
            if t >= 0.0 {
                // Calculates ray hit position and normal
                let position = ray.origin() + ray.direction() * t;
                let normal = self.normal.normalize();

                Some(RayHit::new(
                    position,
                    normal,
                    t,
                    self.color,
                    self.reflection_and_refraction,
                ))
            } else {
                None
            }
        } else {
            None
        }
    }
}
