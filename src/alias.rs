use rand::Rng;
pub use vecmat::vec::*;

pub type Color = Vec3<f64>;
pub type Point = Vec3<f64>;
pub type Vector = Vec3<f64>;
pub type Quaternion = Vec4<f64>;

pub trait Vec2Ext<T>
where
    T: Copy + Default,
{
    fn near_zero(&self) -> bool;
}

impl Vec2Ext<f64> for Vec2<f64> {
    fn near_zero(&self) -> bool {
        // Return true if the vector is close to zero in all dimensions.
        self.map(|i| i.abs() < 1e-8).all()
    }
}

pub trait Vec3Ext<T>
where
    T: Copy + Default,
{
    fn random_vector() -> Vec3<T>;
    fn random_in_unit_sphere() -> Vec3<T>;
    fn random_unit_vector() -> Vec3<T>;
    fn random_in_unit_disk() -> Vec3<T>;
    fn random_cosine_direction() -> Vec3<T>;
    fn random_to_sphere(radius: f64, distance_squared: f64) -> Vec3<T>;
    fn near_zero(&self) -> bool;
    fn reflect(&self, n: &Vector) -> Vector;
    fn refract(&self, n: &Vector, etai_over_etat: f64) -> Vector;
    fn rotate(&self, q: &Quaternion) -> Vector;
}

impl Vec3Ext<f64> for Vec3<f64> {
    fn random_vector() -> Vector {
        Self::from(rand::random(), rand::random(), rand::random())
    }

    fn random_in_unit_sphere() -> Point {
        let mut rng = rand::thread_rng();
        loop {
            let p = Point::from(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
            );
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
            let p = Point::from(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
            if p.sqrlen() < 1.0 {
                return p;
            }
        }
    }

    fn random_cosine_direction() -> Vector {
        let mut rng = rand::thread_rng();
        let r1 = rng.gen::<f64>();
        let r2 = rng.gen::<f64>();
        let z = (1.0 - r2).sqrt();

        let phi = 2.0 * std::f64::consts::PI * r1;
        let x = phi.cos() * r2.sqrt();
        let y = phi.sin() * r2.sqrt();
        Vector::from(x, y, z)
    }

    fn random_to_sphere(radius: f64, distance_squared: f64) -> Vector {
        let mut rng = rand::thread_rng();
        let r1 = rng.gen::<f64>();
        let r2 = rng.gen::<f64>();
        let z = 1.0 + r2 * ((1.0 - radius * radius / distance_squared).sqrt() - 1.0);

        let phi = 2.0 * std::f64::consts::PI * r1;
        let x = phi.cos() * (1.0 - z * z).sqrt();
        let y = phi.sin() * (1.0 - z * z).sqrt();
        Vector::from(x, y, z)
    }

    fn near_zero(&self) -> bool {
        // Return true if the vector is close to zero in all dimensions.
        self.map(|i| i.abs() < 1e-8).all()
    }

    fn reflect(&self, n: &Vector) -> Vector {
        *self - 2.0 * self.dot(*n) * *n
    }

    fn refract(&self, n: &Vector, etai_over_etat: f64) -> Vector {
        let cos_theta = (-*self).dot(*n).min(1.0);
        let r_out_perp = etai_over_etat * (*self + cos_theta * *n);
        let r_out_parallel = -(1.0 - r_out_perp.sqrlen()).abs().sqrt() * *n;
        r_out_perp + r_out_parallel
    }

    fn rotate(&self, q: &Quaternion) -> Vector {
        let p = Vec4::<f64>::from(0.0, self[0], self[1], self[2]);
        let result = q.hamiltonian_prod(&p);
        let result = result.hamiltonian_prod(&q.invert());
        Vector::from(result[1], result[2], result[3])
    }
}

pub trait Vec4Ext<T>
where
    T: Copy + Default,
{
    fn new_quaternion(angle: f64, v: Vec3<T>) -> Vec4<T>;
    fn invert(&self) -> Vec4<T>;
    fn hamiltonian_prod(&self, other: &Vec4<T>) -> Vec4<T>;
}

impl Vec4Ext<f64> for Vec4<f64> {
    fn new_quaternion(angle: f64, axis: Vector) -> Quaternion {
        let v = axis.normalize() * (angle / 2.0).sin();
        Quaternion::from((angle / 2.0).cos(), v[0], v[1], v[2])
    }

    fn invert(&self) -> Quaternion {
        Quaternion::from(self[0], -self[1], -self[2], -self[3])
    }

    fn hamiltonian_prod(&self, other: &Quaternion) -> Quaternion {
        Quaternion::from(
            self[0] * other[0] - self[1] * other[1] - self[2] * other[2] - self[3] * other[3],
            self[0] * other[1] + self[1] * other[0] + self[2] * other[3] - self[3] * other[2],
            self[0] * other[2] - self[1] * other[3] + self[2] * other[0] + self[3] * other[1],
            self[0] * other[3] + self[1] * other[2] - self[2] * other[1] + self[3] * other[0],
        )
    }
}

/// Orthonormal Bases
#[derive(Clone)]
pub struct ONB([Vector; 3]);

impl ONB {
    pub fn from_w(w: &Vector) -> ONB {
        let w = w.normalize();
        let a = if w[0].abs() > 0.9 {
            Vector::from(0.0, 1.0, 0.0)
        } else {
            Vector::from(1.0, 0.0, 0.0)
        };
        let v = w.cross(a).normalize();
        let u = w.cross(v);
        ONB([u, v, w])
    }

    pub fn local(&self, a: &Vector) -> Vector {
        self.u() * a[0] + self.v() * a[1] + self.w() * a[2]
    }

    pub fn u(&self) -> Vector {
        self.0[0]
    }

    pub fn v(&self) -> Vector {
        self.0[1]
    }

    pub fn w(&self) -> Vector {
        self.0[2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn test_rotate() {
        let q = Quaternion::new_quaternion(90.0f64.to_radians(), Vector::from(0.0, 1.0, 0.0));
        let p = Vector::from(1.0, 0.0, 0.0);
        assert!((p.rotate(&q) - Vector::from(0.0, 0.0, -1.0)).near_zero());
    }

    #[test_case( 90.0, Vector::from(0.0, 1.0, 0.0), Vector::from(0.0, 0.0, -1.0))]
    #[test_case( 180.0, Vector::from(0.0, 1.0, 0.0), Vector::from(-1.0, 0.0, 0.0))]
    #[test_case(270.0, Vector::from(0.0, 1.0, 0.0), Vector::from(0.0, 0.0, 1.0))]
    #[test_case( 45.0, Vector::from(0.0, 1.0, 0.0), Vector::from(0.7071067811865475, 0.0, -0.7071067811865476))]
    fn test_rotations(angle: f64, axis: Vector, result: Vector) {
        let q = Quaternion::new_quaternion(angle.to_radians(), axis);
        let p = Vector::from(1.0, 0.0, 0.0);
        assert!((p.rotate(&q) - result).near_zero());
    }

    use proptest::prelude::*;
    proptest! {

        #[test]
        fn test_rotate_around_same_axis_does_nothing(angle in 0.0f64..std::f64::consts::PI) {
            let q =
                Quaternion::new_quaternion(angle, Vector::from(0.0, 1.0, 0.0));
            let p = Vector::from(0.0, 1.0, 0.0);
            prop_assert!((p.rotate(&q) - p).near_zero());
        }
    }
}
