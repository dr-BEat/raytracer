use crate::alias::*;
use crate::hittable::*;
use crate::material::*;
use crate::texture::*;

use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

pub fn two_spheres() -> Vec<Hittable> {
    let checker = Texture::new_checker_color(
        Color::from_array([0.2, 0.3, 0.1]),
        Color::from_array([0.9, 0.9, 0.9]),
    );

    vec![
        Hittable::new_sphere(
            Point::from_array([0.0, -10.0, 0.0]),
            10.0,
            Material::new_lambertian_with_texture(checker.clone()),
        ),
        Hittable::new_sphere(
            Point::from_array([0.0, 10.0, 0.0]),
            10.0,
            Material::new_lambertian_with_texture(checker.clone()),
        ),
    ]
}

pub fn small_scene() -> Vec<Hittable> {
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
    let sphere_left = Hittable::new_sphere(
        Point::from_array([-1.0, 0.0, -1.0]),
        0.5,
        material_left.clone(),
    );
    let sphere_left_inner =
        Hittable::new_sphere(Point::from_array([-1.0, 0.0, -1.0]), -0.4, material_left);
    let sphere_right =
        Hittable::new_sphere(Point::from_array([1.0, 0.0, -1.0]), 0.5, material_right);

    vec![
        sphere_ground,
        sphere_center,
        sphere_left,
        sphere_left_inner,
        sphere_right,
    ]
}

pub fn random_scene() -> Vec<Hittable> {
    let checker = Texture::new_checker_color(
        Color::from_array([0.2, 0.3, 0.1]),
        Color::from_array([0.9, 0.9, 0.9]),
    );
    let mut hittables: Vec<Hittable> = vec![Hittable::new_sphere(
        Point::from_array([0.0, -1000.0, 0.0]),
        1000.0,
        Material::new_lambertian_with_texture(checker),
    )];

    let mut rng = StdRng::seed_from_u64(5);
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center = Point::from_array([
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            ]);

            if (center - Point::from_array([4.0, 0.2, 0.0])).length() > 0.9 {
                let sphere_material = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vector::random_vector() * Vector::random_vector();
                    Material::new_lambertian(albedo)
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vector::from_array([
                        rng.gen_range(0.5, 1.0),
                        rng.gen_range(0.5, 1.0),
                        rng.gen_range(0.5, 1.0),
                    ]);
                    let fuzz = rng.gen_range(0.0, 0.5);
                    Material::new_metal(albedo, fuzz)
                } else {
                    // glass
                    Material::new_dielectric(1.5)
                };

                let hittable = if choose_mat < 0.8 {
                    let center2 = center + Vector::from_array([0.0, rng.gen_range(0.0, 0.5), 0.0]);
                    Hittable::new_moving_sphere(center, center2, 0.0, 1.0, 0.2, sphere_material)
                } else {
                    Hittable::new_sphere(center, 0.2, sphere_material)
                };
                hittables.push(hittable);
            }
        }
    }

    hittables.push(Hittable::new_sphere(
        Point::from_array([0.0, 1.0, 0.0]),
        1.0,
        Material::new_dielectric(1.5),
    ));

    hittables.push(Hittable::new_sphere(
        Point::from_array([-4.0, 1.0, 0.0]),
        1.0,
        Material::new_lambertian(Color::from_array([0.4, 0.2, 0.1])),
    ));

    hittables.push(Hittable::new_sphere(
        Point::from_array([4.0, 1.0, 0.0]),
        1.0,
        Material::new_metal(Color::from_array([0.7, 0.6, 0.5]), 0.0),
    ));

    hittables
}

pub fn earth() -> Vec<Hittable> {
    let earth_texture = Texture::new_image("assets/earthmap.jpg").unwrap();
    let earth_surface = Material::new_lambertian_with_texture(earth_texture);
    let globe = Hittable::new_sphere(Point::new(), 2.0, earth_surface);

    vec![globe]
}

pub fn simple_light() -> Vec<Hittable> {
    vec![
        Hittable::new_sphere(
            Point::from_array([0.0, -1000.0, 0.0]),
            1000.0,
            Material::new_lambertian(Color::from_array([0.8, 0.8, 0.0])),
        ),
        Hittable::new_constant_medium(
            Hittable::new_sphere(
                Point::from_array([0.0, 2.0, 0.0]),
                2.0,
                Material::new_lambertian(Color::from_array([0.8, 0.0, 0.0])),
            ),
            0.91,
            Color::from_array([0.0, 0.0, 0.0]),
        ),
        Hittable::new_sphere(
            Point::from_array([0.0, 2.0, 3.0]),
            1.0,
            Material::new_diffuse_light(Color::from_array([4.0, 4.0, 4.0])),
        ),
        Hittable::new_rotate(
            Hittable::new_cube(
                Point::from_array([0.0, 1.0, -1.7]),
                Point::from_array([4.0, 2.0, -1.6]),
                Material::new_diffuse_light(Color::from_array([4.0, 4.0, 4.0])),
            ),
            5.0f64.to_radians(),
            Vector::from_array([0.0, 0.0, 1.0]),
        ),
        Hittable::new_sphere(
            Point::from_array([2.0, 0.3, 1.0]),
            0.2,
            Material::new_dielectric(1.5),
        ),
    ]
}

pub fn cube_scene() -> Vec<Hittable> {
    vec![Hittable::new_rotate(
        Hittable::new_cube(
            Point::from_array([-1.0, -1.0, -1.0]),
            Point::from_array([1.0, 1.0, 1.0]),
            Material::Lambertian(Texture::Normal),
        ),
        -40.0f64.to_radians(),
        Vector::from_array([0.0, 0.0, 1.0]),
    )]
}
