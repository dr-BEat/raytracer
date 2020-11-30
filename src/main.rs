extern crate bmp;
use bmp::{Image, Pixel};

use rand::Rng;
use std::io;
use std::io::Write;

mod alias;
use alias::*;

mod ray;
use ray::*;

mod cam;
use cam::*;

mod hittable;
use hittable::*;

mod hittablelist;
use hittablelist::*;

mod sphere;
use sphere::Sphere;

mod material;
use crate::material::*;

fn pixel_from_color(color: Color) -> Pixel {
    // gamma-correct for gamma=2.0
    let r = color[0].sqrt();
    let g = color[1].sqrt();
    let b = color[2].sqrt();

    // Write the translated [0,255] value of each color component.
    Pixel::new(
        (r.min(0.9999).max(0.0) * 256.0) as u8,
        (g.min(0.9999).max(0.0) * 256.0) as u8,
        (b.min(0.9999).max(0.0) * 256.0) as u8,
    )
}

fn ray_color(r: &Ray, world: &dyn Hittable, depth: u32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Color::new();
    }

    let hit_result = world.hit(r, 0.001, f64::INFINITY);
    if let Some(hit) = hit_result {
        if let Some((attenuation, scattered)) = hit.material.scatter(r, &hit) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Color::new();
    }

    // Background
    let unit_direction = r.direction.normalize();
    let t = 0.5 * (unit_direction[1] + 1.0);
    (1.0 - t) * Color::from_array([1.0, 1.0, 1.0]) + t * Color::from_array([0.5, 0.7, 1.0])
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 640;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 10;
    let max_depth = 50;

    println!("{} {}", image_width, image_height);

    // World
    let material_ground = Lambertian::new(Color::from_array([0.8, 0.8, 0.0]));
    let material_center = Lambertian::new(Color::from_array([0.1, 0.2, 0.5]));
    let material_left = Dielectric::new(1.5);
    let material_right = Metal::new(Color::from_array([0.8, 0.6, 0.2]), 1.0);

    let sphere_ground = Sphere::new(
        Point::from_array([0.0, -100.5, -1.0]),
        100.0,
        &material_ground,
    );
    let sphere_center = Sphere::new(Point::from_array([0.0, 0.0, -1.0]), 0.5, &material_center);
    let sphere_left = Sphere::new(Point::from_array([-1.0, 0.0, -1.0]), 0.5, &material_left);
    let sphere_right = Sphere::new(Point::from_array([1.0, 0.0, -1.0]), 0.5, &material_right);

    let world = HittableList(vec![
        &sphere_ground,
        &sphere_center,
        &sphere_left,
        &sphere_right,
    ]);

    // Camera
    let cam = Camera::new(90.0, aspect_ratio);

    let mut img = Image::new(image_width, image_height);
    let mut rng = rand::thread_rng();
    for (x, y) in img.coordinates() {
        if x == 0 {
            print!(
                "\rScanlines remaining: {:>3}%",
                100 - 100 * y / image_height
            );
            io::stdout().flush().ok().expect("Could not flush stdout");
        }
        let mut pixel_color = Color::new();
        for _ in 0..samples_per_pixel {
            let u = (x as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
            let v = ((image_height - y - 1) as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;
            let r = cam.get_ray(u, v);
            pixel_color += ray_color(&r, &world, max_depth);
        }
        pixel_color /= samples_per_pixel as f64;
        img.set_pixel(x, y, pixel_from_color(pixel_color));
    }
    println!("\rScanlines remaining:   0%");
    let _ = img.save("img.bmp");
    println!("Created img.bmp");
}
