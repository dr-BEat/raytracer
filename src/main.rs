extern crate image;
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
use crate::material::*;

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

fn ray_color(r: &Ray, world: &Hittable, depth: u32) -> Color {
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
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 500;
    let max_depth = 50;

    println!("{} {}", image_width, image_height);

    // World
    let material_ground = Material::new_lambertian(Color::from_array([0.8, 0.8, 0.0]));
    let material_center = Material::new_lambertian(Color::from_array([0.1, 0.2, 0.5]));
    let material_left = Material::new_dielectric(1.5);
    let material_right = Material::new_metal(Color::from_array([0.8, 0.6, 0.2]), 1.0);

    let sphere_ground = Hittable::new_sphere(
        Point::from_array([0.0, -100.5, -1.0]),
        100.0,
        material_ground,
    );
    let sphere_center =
        Hittable::new_sphere(Point::from_array([0.0, 0.0, -1.0]), 0.5, material_center);
    let sphere_left =
        Hittable::new_sphere(Point::from_array([-1.0, 0.0, -1.0]), 0.5, material_left);
    let sphere_left_inner =
        Hittable::new_sphere(Point::from_array([-1.0, 0.0, -1.0]), -0.4, material_left);
    let sphere_right =
        Hittable::new_sphere(Point::from_array([1.0, 0.0, -1.0]), 0.5, material_right);

    let world = Hittable::List(vec![
        sphere_ground,
        sphere_center,
        sphere_left,
        sphere_left_inner,
        sphere_right,
    ]);

    // Camera
    let lookfrom = Point::from_array([13.0, 2.0, 3.0]);
    let lookat = Point::from_array([0.0, 0.0, 0.0]);
    let vup = Vector::from_array([0.0, 1.0, 0.0]);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    let mut image: RgbImage = ImageBuffer::new(image_width, image_height);
    let now = Instant::now();
    let pixels = (0..image_width)
        .flat_map(move |x| (0..image_height).map(move |y| (x, y)))
        .collect::<Vec<_>>()
        .par_iter()
        .map(|(x, y)| {
            let mut pixel_color = Color::new();
            for _ in 0..samples_per_pixel {
                let u = ((*x) as f64 + rand::random::<f64>()) / (image_width - 1) as f64;
                let v = ((image_height - y - 1) as f64 + rand::random::<f64>())
                    / (image_height - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_depth);
            }
            pixel_color /= samples_per_pixel as f64;
            ((*x, *y), pixel_from_color(pixel_color))
        })
        .collect::<Vec<_>>();

    for pixel in pixels {
        image.put_pixel(pixel.0 .0, pixel.0 .1, pixel.1);
    }
    println!("Rendered in {} seconds", now.elapsed().as_secs_f32());
    image.save("image.png").unwrap();
    println!("Created img.png");
}
