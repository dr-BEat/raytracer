use rand::Rng;
pub use vecmat::vec::*;

pub type Color = Vec3<f64>;
pub type Point = Vec3<f64>;
pub type Vector = Vec3<f64>;

pub fn random_in_unit_sphere() -> Point {
    let mut rng = rand::thread_rng();
    loop {
        let p = Point::from_array([rng.gen(), rng.gen(), rng.gen()]);

        if p.sqrlen() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector() -> Vector {
    random_in_unit_sphere().normalize()
}

pub fn near_zero(v: &Vector) -> bool {
    // Return true if the vector is close to zero in all dimensions.
    let s = 1e-8;
    (v[0].abs() < s) && (v[1].abs() < s) && (v[2].abs() < s)
}

pub fn reflect(v: &Vector, n: &Vector) -> Vector {
    return *v - 2.0 * v.dot(*n) * *n;
}
