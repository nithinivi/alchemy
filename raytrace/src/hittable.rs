// hittable.rs

use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{dot, Point3, Vec3};

pub struct HitRecord {
    pub t: f64,
    pub p: Point3,
    pub normal: Vec3,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(t: f64, p: Point3, normal: Vec3) -> HitRecord {
        HitRecord {
            t,
            p,
            normal,
            front_face: true,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray) {
        self.front_face = dot(r.direction, self.normal) < 0.0;
        if !self.front_face {
            self.normal = -self.normal
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, interval: Interval) -> Option<HitRecord>;
}
