#![warn(clippy::all)]

extern crate image;
use clap::Clap;
use image::ImageBuffer;
use image::Rgb;
use image::RgbImage;
use rayon::prelude::*;
use std::time::Instant;

mod alias;
use alias::*;

mod ray;
use ray::*;

mod cam;
use cam::*;

mod hittable;
use hittable::*;

mod material;

mod aabb;

mod texture;

mod scenes;
use crate::scenes::*;

/// A cool raytracer!
#[derive(Clap)]
#[clap(version = "1.0", author = "BEat")]
struct Opts {
    /// Sets an output file to render too.
    #[clap(short, long, default_value = "renders/image.png")]
    output: String,
    #[clap(short, long, default_value = "4")]
    scene: u32,

    #[clap(short, long, default_value = "1200")]
    image_width: u32,

    #[clap(short, long, default_value = "1.5")]
    aspect_ratio: f64,

    #[clap(long, default_value = "500")]
    samples_per_pixel: u32,

    #[clap(long, default_value = "5")]
    max_depth: u32,
}

fn pixel_from_color(color: Color) -> Rgb<u8> {
    // gamma-correct for gamma=2.0
    let r = color[0].sqrt();
    let g = color[1].sqrt();
    let b = color[2].sqrt();

    // Write the translated [0,255] value of each color component.
    Rgb([
        (r.min(0.9999).max(0.0) * 256.0) as u8,
        (g.min(0.9999).max(0.0) * 256.0) as u8,
        (b.min(0.9999).max(0.0) * 256.0) as u8,
    ])
}

fn ray_color(r: &Ray, background: Color, world: &Hittable, depth: u32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth == 0 {
        return Color::new();
    }

    let hit_result = world.hit(r, 0.001, f64::INFINITY);
    if let Some(hit) = hit_result {
        let emitted = hit.material.emit(hit.u, hit.v, &hit.p);
        if let Some((attenuation, scattered)) = hit.material.scatter(r, &hit) {
            return emitted + attenuation * ray_color(&scattered, background, world, depth - 1);
        }
        return emitted;
    }

    background
}

fn main() {
    let opts: Opts = Opts::parse();

    // Image
    let image_width = opts.image_width;
    let image_height = (image_width as f64 / opts.aspect_ratio) as u32;

    println!("{} {}", image_width, image_height);

    // World
    let mut world = match opts.scene {
        0 => random_scene(),
        1 => two_spheres(),
        2 => earth(),
        3 => simple_light(),
        4 => cube_scene(),
        _ => small_scene(),
    };
    let world = Hittable::new_bvh(world.as_mut_slice(), 0.0, 1.0);

    // Camera
    let lookfrom = Point::from_array([13.0, 2.0, 3.0]);
    let lookat = Point::from_array([0.0, 0.0, 0.0]);
    let vup = Vector::from_array([0.0, 1.0, 0.0]);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let background = Color::from_array([0.50, 0.70, 1.00]);
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        opts.aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    let mut image: RgbImage = ImageBuffer::new(image_width, image_height);
    let now = Instant::now();
    let pixels = (0..image_width)
        .flat_map(move |x| (0..image_height).map(move |y| (x, y)))
        .collect::<Vec<_>>()
        .par_iter()
        .map(|(x, y)| {
            let mut pixel_color = Color::new();
            for _ in 0..opts.samples_per_pixel {
                let u = ((*x) as f64 + rand::random::<f64>()) / (image_width - 1) as f64;
                let v = ((image_height - y - 1) as f64 + rand::random::<f64>())
                    / (image_height - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, background, &world, opts.max_depth);
            }
            pixel_color /= opts.samples_per_pixel as f64;
            ((*x, *y), pixel_from_color(pixel_color))
        })
        .collect::<Vec<_>>();

    for pixel in pixels {
        image.put_pixel(pixel.0 .0, pixel.0 .1, pixel.1);
    }
    println!("Rendered in {} seconds", now.elapsed().as_secs_f32());
    image.save(opts.output).unwrap();
    println!("Created image!");
}
