use crate::intersectable::Intersectable;
use crate::ray::Ray;
use crate::rayhit::{RayHit, ReflectionRefractionIndex};
use crate::vector::Vector3;

/// A sphere with a position, color, and radius
#[derive(Debug)]
pub struct Sphere {
    position: Vector3,
    color: Vector3,
    radius: f32,
    reflection_and_refraction: ReflectionRefractionIndex,
}

impl Sphere {
    /// Creates a new sphere with given geometric data
    pub fn new(
        position: Vector3,
        color: Vector3,
        radius: f32,
        reflection_and_refraction: ReflectionRefractionIndex,
    ) -> Sphere {
        Sphere {
            position,
            color,
            radius,
            reflection_and_refraction,
        }
    }
}

impl Intersectable for Sphere {
    /// Determines whether the given ray has intersected with the sphere
    /// and generates a RayHit
    fn intersect(&self, ray: Ray) -> Option<RayHit> {
        let mut t0;
        let mut t1;

        // Generates a quadratic formula describing the intersection
        let ray_to_sphere = ray.origin() - self.position;
        let a = ray.direction().dot(ray.direction());
        let b = ray.direction().dot(ray_to_sphere) * 2.0;
        let c = ray_to_sphere.dot(ray_to_sphere) - self.radius;

        // Solves the quadratic formula
        let discr = b * b - 4.0 * a * c;
        if discr < 0.0 {
            // If the discriminant < 0, there was no intersection
            return None;
        } else if discr == 0.0 {
            // Single hit
            t0 = -0.5 * b / a;
            t1 = t0;
        } else {
            // Two hits
            let q = if b > 0.0 {
                -0.5 * (b + discr.sqrt())
            } else {
                -0.5 * (b - discr.sqrt())
            };
            t0 = q / a;
            t1 = c / q;
        }

        // Obtains the closer hit
        if t0 > t1 {
            let temp = t0;
            t0 = t1;
            t1 = temp;
        }

        // Ensures that the sphere is in front of the ray's origin
        if t0 < 0.0 {
            t0 = t1;
            if t0 < 0.0 {
                return None;
            }
        }

        // Calculates ray hit position and normal
        let position = ray.origin() + ray.direction() * t0;
        let normal = (position - self.position).normalize();

        Some(RayHit::new(
            position,
            normal,
            t0,
            self.color,
            self.reflection_and_refraction,
        ))
    }
}
