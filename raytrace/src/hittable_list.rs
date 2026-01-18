// hittable_list.rs

use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>, // arr of pointers
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: impl Hittable + 'static) {
        // 'static = WTF?!!>
        self.objects.push(Box::new(object));
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let mut rec = None;
        let mut closest_so_fat = ray_tmax;

        for object in self.objects.iter() {
            if let Some(hrec) = object.hit(r, ray_tmin, closest_so_fat) {
                closest_so_fat = hrec.t;
                rec = Some(hrec);
            }
        }
        rec
    }
}
