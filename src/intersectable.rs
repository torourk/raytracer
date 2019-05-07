use crate::ray::Ray;
use crate::rayhit::RayHit;

/// Intersectable defines behavior for objects that can be seen
/// by the Ray Tracer
pub trait Intersectable {

    /// Returns whether the ray hit the Intersectable in the form of
    /// a RayHit or None if there was no intersection
    fn intersect(&self, ray: Ray) -> Option<RayHit>;
}