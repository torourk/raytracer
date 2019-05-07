use std::fs::File;
use std::io::BufWriter;
use std::time::Instant;

use png::{BitDepth, ColorType, Encoder, HasParameters};
use scoped_threadpool::Pool;

use camera::Camera;
use intersectable::Intersectable;
use pixel::{IntoPixelData, Pixel};
use plane::Plane;
use sphere::Sphere;
use trace::trace_chunk;
use vector::Vector3;

mod camera;
mod intersectable;
mod pixel;
mod plane;
mod ray;
mod rayhit;
mod sphere;
mod trace;
mod vector;

// Image output size
const WIDTH: u32 = 2048;
const HEIGHT: u32 = 2048;

// Number of threads for thread pool
const THREADS: u32 = 4;

fn main() {
    // Creates the camera
    let camera = Camera::new(
        Vector3::new(-0.5, -0.5, -3.0),
        Vector3::new(0.0, 1.0, 0.0),
        90.0,
        0.0,
        5.0,
    );

    // Creates world objects
    let sphere = Box::new(Sphere::new(
        Vector3::new(-0.75, -0.125, -0.5),
        Vector3::new(0.0, 0.0, 0.75),
        0.75,
        Some((0.5, Some(1.1))),
    ));
    let sphere2 = Box::new(Sphere::new(
        Vector3::new(0.5, 0.0, 1.0),
        Vector3::new(0.0, 1.0, 1.0),
        1.0,
        Some((0.5, None)),
    ));
    let sphere3 = Box::new(Sphere::new(
        Vector3::new(1.0, 2.0, 2.0),
        Vector3::new(1.0, 0.0, 0.0),
        1.0,
        Some((0.5, None)),
    ));
    let plane = Box::new(Plane::new(
        Vector3::new(0.0, -1.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        Vector3::new(0.5, 0.5, 0.5),
        Some((0.5, None)),
    ));

    // Brings objects into a single array
    let mut world: Vec<Box<Intersectable + Sync + Send>> = Vec::new();
    world.push(sphere);
    world.push(sphere2);
    world.push(sphere3);
    world.push(plane);

    // Creates a pixel array large enough for the output image
    let mut data: Vec<Pixel> = vec![Pixel::new(0, 0, 0, 0); (WIDTH * HEIGHT) as usize];

    // Starts timer
    let trace_start = Instant::now();

    // Creates thread pool for ray tracing
    let mut pool = Pool::new(THREADS);
    pool.scoped(|scope| {
        let thread_world = &world;

        // Keeps track of which pixels have been sent to the pool so far
        let mut start = 0;

        // Divides the data array into chunks that will store our data
        // We create a chunk for each thread allocated on the pool
        for chunk in data.chunks_mut((WIDTH * HEIGHT / THREADS) as usize) {
            let chunk_len = chunk.len();

            // Executes the trace
            scope.execute(move || {
                trace_chunk(chunk, start as u32, WIDTH, HEIGHT, &camera, thread_world);
            });

            start += chunk_len;
        }
    });
    // Thread pool is effectively joined here

    // Stops the timer
    let trace_duration = trace_start.elapsed().as_millis();

    // Writes the image
    let draw_start = Instant::now();
    write_image(data);
    let draw_duration = draw_start.elapsed().as_millis();

    println!("Trace: {} ms.", trace_duration);
    println!("Draw:  {} ms.", draw_duration);
}

/// Writes the pixel array to an image
fn write_image(data: Vec<Pixel>) {
    let file = File::create("out.png").unwrap();
    let w = BufWriter::new(file);

    let mut encoder = Encoder::new(w, WIDTH, HEIGHT);
    encoder.set(ColorType::RGBA).set(BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    let pixel_data = data.into_pixel_data();

    writer.write_image_data(&pixel_data).unwrap();
}
