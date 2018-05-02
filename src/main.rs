//
extern crate rand;

pub mod vec3;
pub mod ray;
pub mod material;

use std::io::Write;
use std::fs::File;
use std::path::Path;
use std::f32;
//use rand::Rng;

use vec3::*;
use ray::*;
use material::*;

trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}

struct Sphere {
    centre: Vec3,
    radius: f32,
    material: Box<Material>
}

impl Sphere {
    fn new(c: Vec3, r: f32, m:Box<Material>) -> Sphere { Sphere { centre:c, radius:r, material:m} }
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

struct HitableList {
    list: Vec<Box<Hitable>>
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

struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {
    fn new() -> Camera {
        Camera {
            origin:Vec3::new(0.0, 0.0, 0.0), 
            lower_left_corner:Vec3::new(-2.0, -1.0, -1.0),
            horizontal:Vec3::new(4.0, 0.0, 0.0),
            vertical:Vec3::new(0.0, 2.0, 0.0)
        }
    }

    fn get_ray(&self, u: f32, v:f32) -> Ray { Ray::new(self.origin, self.lower_left_corner + u*self.horizontal + v*self.vertical) }
}

fn colour(r: &Ray, world: &Hitable, depth: i32) -> Vec3 {
        
    match world.hit(r, 0.001, f32::MAX) {
        Some(rec) => {
            if depth > 50 {
                return Vec3::zero();
            }

            match rec.m.scatter(r, &rec) {
                Some(s) => { return s.attenuation * colour(&s.scattered, world, depth+1); }
                None => { return Vec3::zero(); }
            }
        }
        
        None => {
            // Sky
            let unit_direction = unit_vector(r.direction());
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;
    
    let cam = Camera::new();

    let world = HitableList {
        list: vec!(
            Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Box::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5))))),
            Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Box::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))))),
            Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, Box::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.4)))),
            Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, Box::new(Dielectric::new(1.5)))),
            Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.45, Box::new(Dielectric::new(1.5))))
        )
    };
  
    let mut file = File::create(Path::new("out.ppm")).expect("can't open");
    
    write!(file, "P6\n{width} {height}\n255\n", width=nx, height=ny).expect("can't write");
    
    for j in (0..ny).rev() {
//        println!("Line: {}", j);
        for i in 0..nx {
            let mut col = Vec3::zero();

            for _s in 0..ns {
                let u = (i as f32 + rand()) / nx as f32;
                let v = (j as f32 +rand()) / ny as f32;
                
                let r = cam.get_ray(u,v);
                col += colour(&r, &world, 0);
            }

            col *= 1.0 / ns as f32;
            col = Vec3::new(col.r().sqrt(), col.g().sqrt(), col.b().sqrt());
            
            let ia  = [ (255.99 * col.r()) as u8, (255.99 * col.g()) as u8, (255.99 * col.b()) as u8];
            file.write(&ia).expect("can't write");
        }
    }
}
