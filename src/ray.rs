use crate::alias::*;

#[derive(Default)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
    pub time: f64,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector, time: f64) -> Self {
        Self {
            origin: origin,
            direction: direction,
            time: time,
        }
    }

    pub fn at(&self, time: f64) -> Point {
        self.origin + time * self.direction
    }
}
