use crate::camera::Camera;
use crate::intersectable::Intersectable;
use crate::pixel::Pixel;
use crate::ray::Ray;
use crate::rayhit::RayHit;
use crate::vector::Vector3;

/// Number of maximum bounces per ray
const BOUNCES: u32 = 8;

/// Calculates a reflection vector given a source vector and normal vector
fn reflect(i: Vector3, n: Vector3) -> Vector3 {
    i - (n * (2.0 * n.dot(i)))
}

/// Calculates a refraction vector given a source vector, normal vector, and
/// index of refraction
fn refract(i: Vector3, n: Vector3, ior: f32) -> Vector3 {
    let mut cosi = clamp(i.dot(n), -1.0, 1.0);
    let mut etai = 1.0;
    let mut etat = ior;
    let mut normal = n;

    if cosi < 0.0 {
        cosi = -cosi;
    } else {
        let temp = etai;
        etai = etat;
        etat = temp;
        normal = -normal;
    }
    let eta = etai / etat;
    let k = 1.0 - eta * eta * (1.0 - cosi * cosi);
    let mut out = Vector3::origin();
    if k >= 0.0 {
        out = (i * eta) + (normal * (eta * cosi - k.sqrt()));
    }
    out
}

/// Clamps a float between two given floats
fn clamp(input: f32, min: f32, max: f32) -> f32 {
    input.max(min).min(max)
}

/// Calculates Fresnel reflective transmittance
fn fresnel(i: Vector3, n: Vector3, ior: f32) -> f32 {
    let mut cosi = clamp(i.dot(n), -1.0, 1.0);
    let mut etai = 1.0;
    let mut etat = ior;
    let kr;
    if cosi > 0.0 {
        etai = ior;
        etat = 1.0;
    }
    let sint = etai / etat * 0.0f32.max(1.0 - cosi * cosi).sqrt();
    if sint >= 1.0 {
        kr = 1.0;
    } else {
        let cost = 0.0f32.max(1.0 - sint * sint).sqrt();
        cosi = cosi.abs();
        let rs = ((etat * cosi) - (etai * cost)) / ((etat * cosi) + (etai * cost));
        let rp = ((etai * cosi) - (etat * cost)) / ((etai * cosi) + (etat * cost));
        kr = (rs * rs + rp * rp) / 2.0;
    }
    kr
}

/// Traces a ray through the world
fn trace(depth: u32, ray: Ray, world: &Vec<Box<Intersectable + Sync + Send>>) -> Vector3 {
    // Ambient light strength
    let ambient_strength = 0.1;

    // Light direction vector and color
    let light_dir = (-Vector3::new(-1.0, -1.0, 1.0)).normalize();
    let light_color = Vector3::new_scalar(1.0);

    // Specular strength of objects
    let specular_strength = 0.5;

    let mut closest_raycast: Option<RayHit> = None;
    for object in world {
        let raycast = object.intersect(ray);

        // Determines the new closest intersection
        closest_raycast = match raycast {
            Some(hit) => match &closest_raycast {
                Some(closest_hit) => {
                    if hit.distance() < closest_hit.distance() {
                        Some(hit)
                    } else {
                        closest_raycast
                    }
                }
                None => Some(hit),
            },
            None => closest_raycast,
        };
    }

    // Checks if anything was hit
    if closest_raycast.is_none() {
        // Background color
        return Vector3::new(0.529, 0.808, 0.98);
    } else {
        let closest_hit = closest_raycast.unwrap();
        let hit_bias = closest_hit.normal() * 0.001;

        // Calculates shadow ray to see if we're in view of the light source
        let shadow_origin = closest_hit.position() + hit_bias;
        let shadow_ray = Ray::new(shadow_origin, light_dir);

        let mut closest_shadow_raycast: Option<RayHit> = None;
        for object in world {
            let shadow_raycast = object.intersect(shadow_ray);

            // Determines the new closest intersection
            closest_shadow_raycast = match shadow_raycast {
                Some(shadow_hit) => match &closest_shadow_raycast {
                    Some(closest_shadow_hit) => {
                        if shadow_hit.distance() < closest_shadow_hit.distance() {
                            Some(shadow_hit)
                        } else {
                            closest_shadow_raycast
                        }
                    }
                    None => Some(shadow_hit),
                },
                None => closest_shadow_raycast,
            };
        }

        // Final output color
        let mut out_float;

        // Gets hit information and calculates ambient light
        let color = closest_hit.color();
        let normal = closest_hit.normal();
        let ambient = color * ambient_strength;

        // Checks if we're in view of the light source
        if closest_shadow_raycast.is_some() {
            // View of light blocked
            out_float = ambient;
        } else {
            // Calculates diffuse and specular lighting
            let halfway_dir = (light_dir - ray.direction()).normalize();
            let diffuse = color * normal.dot(light_dir).max(0.0) * light_color;
            let specular =
                light_color * specular_strength * normal.dot(halfway_dir).max(0.0).powf(64.0);
            out_float = diffuse + specular + ambient;
        }

        // Checks if this raycast exceeds our bounce limit
        if depth < BOUNCES {
            // Gets reflect and refract information
            let reflect_and_refract = closest_hit.reflection_and_refraction_index();

            // Checks if the surface is reflectable
            if let Some((reflect_index, refract_option)) = reflect_and_refract {
                // Checks if we're outside the surface
                let outside = ray.direction().dot(closest_hit.normal()) < 0.0;

                // Calculates the reflection vector and traces it
                let reflect_origin = if outside {
                    closest_hit.position() + hit_bias
                } else {
                    closest_hit.position() - hit_bias
                };
                let reflect_ray =
                    Ray::new(reflect_origin, reflect(ray.direction(), normal).normalize());
                let reflection_color = trace(depth + 1, reflect_ray, world);

                // Checks if the surface is refractable
                if let Some(refract_index) = refract_option {
                    // Calculates the reflective transmittance
                    let kr = fresnel(ray.direction(), normal, refract_index);
                    let mut refraction_color = Vector3::origin();

                    // Checks if the surface has total internal reflection
                    if kr < 1.0 {
                        // Calculates the refraction vector and traces it
                        let refract_origin = if outside {
                            closest_hit.position() - hit_bias
                        } else {
                            closest_hit.position() + hit_bias
                        };
                        let refract_ray = Ray::new(
                            refract_origin,
                            refract(ray.direction(), normal, refract_index).normalize(),
                        );
                        refraction_color = trace(depth + 1, refract_ray, world);
                    }

                    // Adds the reflection and refraction color information
                    out_float = out_float + reflection_color * kr + refraction_color * (1.0 - kr);
                } else {
                    // Adds the reflection color information
                    out_float = out_float + reflection_color * reflect_index;
                }
            }
        }
        out_float
    }
}

/// Traces a given pixel of the viewport
fn trace_pixel(
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    camera: &Camera,
    world: &Vec<Box<Intersectable + Sync + Send>>,
) -> Pixel {
    // Calculates viewport information
    let aspect = width as f32 / height as f32;
    let cam_right = camera.up().cross(camera.direction().normalize());
    let fov_radians = std::f32::consts::PI * (camera.fov() / 2.0) / 180.0;
    let half_width = fov_radians.tan();
    let half_height = (1.0 / aspect as f32) * half_width;
    let camera_width = half_width * 2.0;
    let camera_height = half_height * 2.0;
    let pixel_width = camera_width / (width - 1) as f32;
    let pixel_height = camera_height / (height - 1) as f32;

    // Calculates x and y vector offsets
    let x_vec = cam_right * (x as f32 * pixel_width - half_width);
    let y_vec = -camera.up() * (y as f32 * pixel_height - half_height);

    // Generates pixel ray
    let ray = Ray::new(
        camera.position(),
        (camera.direction() + x_vec + y_vec).normalize(),
    );

    // Traces the ray, converts the color from 0..1 to 0..256, and returns it
    let mut color = trace(0, ray, world) * 255.0;
    color.x = color.x.min(255.0);
    color.y = color.y.min(255.0);
    color.z = color.z.min(255.0);
    Pixel::new(color.x as u8, color.y as u8, color.z as u8, 255)
}

/// Traces a given chunk of pixels
pub fn trace_chunk(
    chunk: &mut [Pixel],
    start: u32,
    width: u32,
    height: u32,
    camera: &Camera,
    world: &Vec<Box<Intersectable + Sync + Send>>,
) {
    for i in start..(start + chunk.len() as u32) {
        chunk[(i - start) as usize] =
            trace_pixel(i % width, i / width, width, height, camera, world);
    }
}
