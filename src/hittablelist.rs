use crate::hittable::*;
use crate::ray::*;

pub struct HittableList(pub Vec<Box<dyn Hittable>>);

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut record = None;
        let mut closest_so_far = t_max;
        for hittable in &self.0 {
            if let Some(new_record) = hittable.hit(r, t_min, closest_so_far) {
                if new_record.t < closest_so_far {
                    closest_so_far = new_record.t;
                    record = Some(new_record);
                }
            }
        }
        record
    }
}
