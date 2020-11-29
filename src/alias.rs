use rand::Rng;
pub use vecmat::vec::*;

pub type Color = Vec3<f64>;
pub type Point = Vec3<f64>;
pub type Vector = Vec3<f64>;

pub fn random_vector() -> Vector {
    Vector::from_array([rand::random(), rand::random(), rand::random()])
}

pub fn random_in_unit_sphere() -> Point {
    let mut rng = rand::thread_rng();
    loop {
        let p = Point::from_array([rng.gen(), rng.gen(), rng.gen()]);

        if p.sqrlen() < 1.0 {
            return p;
        }
    }
}
