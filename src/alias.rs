use rand::Rng;
pub use vecmat::vec::*;

pub type Color = Vec3<f64>;
pub type Point = Vec3<f64>;
pub type Vector = Vec3<f64>;

pub fn random_vector() -> Vector {
    Vector::from_array([
        rand::random::<f64>(),
        rand::random::<f64>(),
        rand::random::<f64>(),
    ])
}

pub fn random_in_unit_sphere() -> Point {
    let mut rng = rand::thread_rng();
    loop {
        let p = Point::from_array([
            rng.gen_range(-1.0, 1.0),
            rng.gen_range(-1.0, 1.0),
            rng.gen_range(-1.0, 1.0),
        ]);

        if p.sqrlen() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector() -> Vector {
    random_in_unit_sphere().normalize()
}

pub fn random_in_unit_disk() -> Point {
    let mut rng = rand::thread_rng();
    loop {
        let p = Point::from_array([rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0), 0.0]);

        if p.sqrlen() < 1.0 {
            return p;
        }
    }
}

pub fn near_zero(v: &Vector) -> bool {
    // Return true if the vector is close to zero in all dimensions.
    let s = 1e-8;
    (v[0].abs() < s) && (v[1].abs() < s) && (v[2].abs() < s)
}

pub fn reflect(v: &Vector, n: &Vector) -> Vector {
    return *v - 2.0 * v.dot(*n) * *n;
}

pub fn refract(uv: &Vector, n: &Vector, etai_over_etat: f64) -> Vector {
    let cos_theta = (-*uv).dot(*n).min(1.0);
    let r_out_perp = etai_over_etat * (*uv + cos_theta * *n);
    let r_out_parallel = -(1.0 - r_out_perp.sqrlen()).abs().sqrt() * *n;
    r_out_perp + r_out_parallel
}
