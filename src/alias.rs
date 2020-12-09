use rand::Rng;
pub use vecmat::vec::*;

pub type Color = Vec3<f64>;
pub type Point = Vec3<f64>;
pub type Vector = Vec3<f64>;

pub trait Vec3Ext<T>
where
    T: Copy + Default,
{
    fn random_vector() -> Vec3<T>;
    fn random_in_unit_sphere() -> Vec3<T>;
    fn random_unit_vector() -> Vec3<T>;
    fn random_in_unit_disk() -> Vec3<T>;
    fn near_zero(&self) -> bool;
    fn reflect(&self, n: &Vector) -> Vector;
    fn refract(&self, n: &Vector, etai_over_etat: f64) -> Vector;
}

impl Vec3Ext<f64> for Vec3<f64> {
    fn random_vector() -> Vector {
        Self::from_array([rand::random(), rand::random(), rand::random()])
    }

    fn random_in_unit_sphere() -> Point {
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

    fn random_unit_vector() -> Vector {
        Self::random_in_unit_sphere().normalize()
    }

    fn random_in_unit_disk() -> Point {
        let mut rng = rand::thread_rng();
        loop {
            let p = Point::from_array([rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0), 0.0]);
            if p.sqrlen() < 1.0 {
                return p;
            }
        }
    }

    fn near_zero(&self) -> bool {
        // Return true if the vector is close to zero in all dimensions.
        let s = 1e-8;
        (self[0].abs() < s) && (self[1].abs() < s) && (self[2].abs() < s)
    }

    fn reflect(&self, n: &Vector) -> Vector {
        return *self - 2.0 * self.dot(*n) * *n;
    }

    fn refract(&self, n: &Vector, etai_over_etat: f64) -> Vector {
        let cos_theta = (-*self).dot(*n).min(1.0);
        let r_out_perp = etai_over_etat * (*self + cos_theta * *n);
        let r_out_parallel = -(1.0 - r_out_perp.sqrlen()).abs().sqrt() * *n;
        r_out_perp + r_out_parallel
    }
}
