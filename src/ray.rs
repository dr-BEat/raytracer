use crate::alias::*;

#[derive(Default)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Self {
            origin: origin,
            direction: direction,
        }
    }

    pub fn at(&self, time: f64) -> Point {
        self.origin + time * self.direction
    }
}
