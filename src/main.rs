extern crate bmp;
use bmp::{Image, Pixel};

mod alias;
use alias::*;

mod ray;
use ray::*;

mod hittable;
use hittable::*;

mod hittablelist;
use hittablelist::*;

mod sphere;
use sphere::Sphere;

fn pixel_from_color(color: Color) -> Pixel {
    Pixel::new(
        (color[0] * 255.9999) as u8,
        (color[1] * 255.9999) as u8,
        (color[2] * 255.9999) as u8,
    )
}

fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    let hit_result = world.hit(r, 0.0, f64::INFINITY);
    if let Some(hit) = hit_result {
        return 0.5 * (hit.normal + Color::from_array([1.0, 1.0, 1.0]));
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

    println!("{} {}", image_width, image_height);

    // World
    let world = HittableList(vec![
        Box::new(Sphere::new(Point::from_array([0.0, 0.0, -1.0]), 0.5)),
        Box::new(Sphere::new(Point::from_array([0.0, -100.5, -1.0]), 100.0)),
    ]);

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point::new();
    let horizontal = Vector::from_array([viewport_width, 0.0, 0.0]);
    let vertical = Vector::from_array([0.0, viewport_height, 0.0]);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vector::from_array([0.0, 0.0, focal_length]);

    let mut img = Image::new(image_width, image_height);

    for (x, y) in img.coordinates() {
        if x == 0 {
            print!(
                "\rScanlines remaining: {:>3}%",
                100 - 100 * y / image_height
            );
        }
        let u = x as f64 / (image_width - 1) as f64;
        let v = (image_height - y - 1) as f64 / (image_height - 1) as f64;
        let r = Ray::new(
            origin,
            lower_left_corner + u * horizontal + v * vertical - origin,
        );
        let pixel_color = ray_color(&r, &world);

        img.set_pixel(x, y, pixel_from_color(pixel_color));
    }
    println!("\rScanlines remaining:   0%");
    let _ = img.save("img.bmp");
    println!("Created img.bmp");
}
