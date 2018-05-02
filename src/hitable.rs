extern crate rand;

use vec3::*;
use ray::*;
use material::*;

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}

pub struct Sphere {
    centre: Vec3,
    radius: f32,
    material: Box<Material>
}

impl Sphere {
    pub fn new(c: Vec3, r: f32, m:Box<Material>) -> Sphere { Sphere { centre:c, radius:r, material:m} }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let oc = r.origin() - self.centre;
        let a = dot(r.direction(), r.direction());
        let b = 2.0 * dot(oc, r.direction());
        let c = dot(oc, oc) - self.radius * self.radius;
        let discriminant = b*b - 4.0*a*c;
        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / (2.0*a);
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                return Some(Hit::new(temp, p, (p - self.centre) / self.radius, &self.material));
            }

            let temp = (-b + discriminant.sqrt()) / (2.0*a);
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                return Some(Hit::new(temp, p, (p - self.centre) / self.radius, &self.material));
            }
        }

        None
    }
}

pub struct HitableList {
    list: Vec<Box<Hitable>>
}

impl HitableList {
    pub fn new(l:Vec<Box<Hitable>>) -> HitableList { HitableList { list:l } }
    pub fn push(&mut self, h: Box<Hitable>) { self.list.push(h); }
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let mut temp_rec = None;
        let mut closest_so_far = t_max;

        for h in &self.list {
            match h.hit(r, t_min, closest_so_far) {
                Some(rec) => {
                    closest_so_far = rec.t;
                    temp_rec = Some(rec);
                }
                None => {
                }
            }
        }
        
        temp_rec
    }
}

