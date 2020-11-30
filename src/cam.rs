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
    pub fn new(vfov: f64, aspect_ratio: f64) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point::new();
        let horizontal = Vector::from_array([viewport_width, 0.0, 0.0]);
        let vertical = Vector::from_array([0.0, viewport_height, 0.0]);
        Self {
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: origin
                - horizontal / 2.0
                - vertical / 2.0
                - Vector::from_array([0.0, 0.0, focal_length]),
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical
                - self.origin,
        }
    }
}
