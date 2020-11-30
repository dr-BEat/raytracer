use crate::alias::*;
use crate::ray::*;

pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vector,
    vertical: Vector,
}

impl Camera {
    /// Returns a camera with the given vfov and aspect_ration
    ///
    /// # Arguments
    ///
    /// * `vfox` - vertical field-of-view in degrees
    pub fn new(lookfrom: Point, lookat: Point, vup: Vector, vfov: f64, aspect_ratio: f64) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        let origin = lookfrom;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        Self {
            origin: lookfrom,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: origin - horizontal / 2.0 - vertical / 2.0 - w,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + s * self.horizontal + t * self.vertical
                - self.origin,
        }
    }
}
