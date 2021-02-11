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
mod aabb;
mod material;
mod pdf;
mod scenes;
mod texture;
use crate::material::*;
use crate::pdf::PDF;
use crate::scenes::*;

/// A cool raytracer!
#[derive(Clap)]
#[clap(version = "1.0", author = "BEat")]
struct Opts {
    /// Sets an output file to render too.
    #[clap(short, long, default_value = "renders/image.png")]
    output: String,
    #[clap(short, long, default_value = "3")]
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
        (r.clamp(0.0, 0.9999) * 256.0) as u8,
        (g.clamp(0.0, 0.9999) * 256.0) as u8,
        (b.clamp(0.0, 0.9999) * 256.0) as u8,
    ])
}

fn ray_color(
    r: &Ray,
    background: Color,
    world: &Hittable,
    lights: &Option<Hittable>,
    depth: u32,
) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth == 0 {
        return Color::new();
    }

    if let Some(hit) = world.hit(r, 0.001, f64::INFINITY) {
        let emitted = hit.material.emit(&hit);
        if let Some(scatter) = hit.material.scatter(r, &hit) {
            match scatter {
                ScatterRecord::Specular { ray, attenuation } => {
                    return attenuation * ray_color(&ray, background, world, lights, depth - 1)
                }
                ScatterRecord::PDF { pdf, attenuation } => {
                    let mut pdf = pdf;
                    if let Some(lights) = lights {
                        let light = PDF::Hittable(lights, hit.p);
                        pdf = PDF::Mixture(vec![light, pdf]);
                    }
                    let scattered = Ray::new(hit.p, pdf.generate(), r.time);
                    let pdf_value = pdf.value(&scattered.direction);

                    return emitted
                        + attenuation
                            * hit.material.scattering_pdf(r, &hit, &scattered)
                            * ray_color(&scattered, background, world, lights, depth - 1)
                            / pdf_value;
                }
            }
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
    let (mut world, mut lights) = match opts.scene {
        0 => random_scene(),
        1 => two_spheres(),
        2 => earth(),
        3 => simple_light(),
        4 => cube_scene(),
        _ => small_scene(),
    };
    let world = Hittable::new_bvh(world.as_mut_slice(), 0.0, 1.0);
    let lights = match lights.as_slice() {
        [] => None,
        [Hittable::Empty] => None,
        [_] => lights.pop(),
        _ => Some(Hittable::List(lights)),
    };

    // Camera
    let lookfrom = Point::from(13.0, 2.0, 3.0);
    let lookat = Point::from(0.0, 0.0, 0.0);
    let vup = Vector::from(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let background = Color::from(0.50, 0.70, 1.00);
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
                pixel_color += ray_color(&r, background, &world, &lights, opts.max_depth);
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
