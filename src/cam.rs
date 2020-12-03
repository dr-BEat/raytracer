use crate::alias::*;
use crate::ray::*;

use rand;
use rand::Rng;

pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vector,
    vertical: Vector,
    u: Vector,
    v: Vector,
    #[allow(dead_code)]
    w: Vector,
    lens_radius: f64,
    time_open: f64,
    time_close: f64,
}

impl Camera {
    /// Returns a camera with the given vfov and aspect_ration
    ///
    /// # Arguments
    ///
    /// * `vfox` - vertical field-of-view in degrees
    pub fn new(
        lookfrom: Point,
        lookat: Point,
        vup: Vector,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        time_open: f64,
        time_close: f64,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        Self {
            origin: lookfrom,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w,
            u: u,
            v: v,
            w: w,
            lens_radius: aperture / 2.0,
            time_open: time_open,
            time_close: time_close,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd[0] + self.v * rd[1];
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
            rand::thread_rng().gen_range(self.time_open, self.time_close),
        )
    }
}
