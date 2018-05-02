//
extern crate rand;

pub mod vec3;
pub mod ray;
pub mod material;

use std::io::Write;
use std::fs::File;
use std::path::Path;
use std::f32;
use std::f32::consts::PI;

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
    vertical: Vec3,

    u:Vec3,
    v:Vec3,
    w:Vec3,

    lens_radius:f32
}

impl Camera {
    fn new(lookfrom:Vec3, lookat:Vec3, vup: Vec3, vfov: f32, aspect:f32, aperture:f32, focus:f32) -> Camera {
        let theta = vfov * PI / 180.0;
        let half_height = (theta/2.0).tan();
        let half_width = aspect * half_height;

        let w = unit_vector(lookfrom - lookat);
        let u = unit_vector(cross(vup, w));
        let v = cross(w, u);
        
        Camera {
            origin: lookfrom,  
            lower_left_corner: lookfrom - focus * (half_width*u + half_height*v + w),
            horizontal: focus * (2.0 * half_width * u),
            vertical: focus * (2.0 * half_height * v),
            u: u, v: v, w: w,
            lens_radius: aperture/2.0
        }
    }

    fn get_ray(&self, u: f32, v:f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(self.origin + offset, self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin - offset) }
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

fn random_scene() -> HitableList {
    let n = 500;

    let mut world = HitableList { list: Vec::new() };
    
    world.list.push(Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Box::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))))));

    for a in -11 .. 11 {
        for b in -11 .. 11 {
            let choose_mat = rand();
            let centre = Vec3::new(a as f32 + 0.9*rand(), 0.2, b as f32 + 0.9 * rand());
            if (centre - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    world.list.push(Box::new(Sphere::new(centre, 0.2,
                                                         Box::new(Lambertian::new(Vec3::new(rand()*rand(), rand()*rand(), rand()*rand()))))));
                } else if choose_mat < 0.95 {
                    // Metal
                    world.list.push(Box::new(Sphere::new(centre, 0.2,
                                                         Box::new(Metal::new(Vec3::new(0.5*(1.0+rand()), 0.5*(1.0+rand()), 0.5*(1.0+rand())), 0.5*rand())))));
                                                          
                } else {
                    // Glass
                    world.list.push(Box::new(Sphere::new(centre, 0.2,Box::new(Dielectric::new(1.5)))));
                }
                   
                    
            }
        }
    }
    world.list.push(Box::new(Sphere::new(Vec3::new(0.0,1.0,0.0), 1.0, Box::new(Dielectric::new(1.5)))));
    world.list.push(Box::new(Sphere::new(Vec3::new(-4.0,1.0,0.0), 1.0, Box::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))))));
    world.list.push(Box::new(Sphere::new(Vec3::new(4.0,1.0,0.0), 1.0, Box::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)))));

    world
}

fn test_scene() -> HitableList {
    HitableList {
        list: vec!(
            Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Box::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5))))),
            Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Box::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))))),
            Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, Box::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.4)))),
            Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, Box::new(Dielectric::new(1.5)))),
            Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.45, Box::new(Dielectric::new(1.5))))
        )
    }
}

fn main() {
    let nx = 800;
    let ny = 600;
    let ns = 100;

    let lookfrom = Vec3::new(14.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 0.05;
    
    let cam = Camera::new(lookfrom, lookat, Vec3::new(0.0,1.0,0.0), 20.0, nx as f32 / ny as f32, aperture, dist_to_focus);

    let world = random_scene();
    
    let mut file = File::create(Path::new("out.ppm")).expect("can't open");
    
    write!(file, "P6\n{width} {height}\n255\n", width=nx, height=ny).expect("can't write");
    
    for j in (0..ny).rev() {
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
