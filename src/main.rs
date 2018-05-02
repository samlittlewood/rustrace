//
extern crate rand;

pub mod vec3;
pub mod ray;

use std::io::Write;
use std::fs::File;
use std::path::Path;
use std::f32;
use rand::Rng;

use vec3::*;
use ray::*;

struct Scattered {
    scattered: Ray,
    attenuation: Vec3,
}

trait Material {
    fn scatter(&self, r_in: &Ray, rec: &Hit) -> Option<Scattered>;
}

struct Lambertian {
    albedo: Vec3
}

impl Lambertian {
    fn new(a : Vec3) -> Lambertian { Lambertian { albedo: a } }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &Hit, ) -> Option<Scattered> {
        let target = rec.p + rec.n + random_in_unit_sphere();
        Some(Scattered { scattered:Ray::new(rec.p, target - rec.p), attenuation:self.albedo })
    }
}

#[derive(Copy, Clone)]
struct Hit {
    t: f32,
    p: Vec3,
    n: Vec3,
//    material: &'a mut Material,
}

impl Hit {
    fn new(nt: f32, np: Vec3, nn: Vec3) -> Hit { Hit { t:nt, p:np, n:nn } }
}

trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}

struct Sphere<'a> {
    centre: Vec3,
    radius: f32,
    material: &'a Material
}

impl<'a, 'b> Sphere<'a, 'b> {
    fn new(c: Vec3, r: f32, m: &'b Material) -> Sphere { Sphere { centre:c, radius:r, material:m } }
}

impl<'a> Hitable for Sphere<'a> {
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
                return Some(Hit::new(temp, p, (p - self.centre) / self.radius));
            }

            let temp = (-b + discriminant.sqrt()) / (2.0*a);
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                return Some(Hit::new(temp, p, (p - self.centre) / self.radius));
            }
        }

        None
    }
}

struct HitableList {
    list: Vec<Box<Hitable>>,
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

fn rand() -> f32 {
    rand::thread_rng().gen_range(0.0, 1.0)
}

fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = (2.0 * Vec3::new(rand(), rand(), rand())) - Vec3::new(1.0,1.0,1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

fn colour(r: &Ray, world: &Hitable, depth: i32) -> Vec3 {
//    if depth > 2 {
//        return Vec3::zero();
//    }
        
    match world.hit(r, 0.001, f32::MAX) {
        Some(rec) => {
            let target = rec.p + rec.n + random_in_unit_sphere();
            0.5 * colour(&Ray::new(rec.p, target - rec.p), world, depth + 1)
        }
        None => {
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
    
    let mut file = File::create(Path::new("out_200x100_25Hz_8bit_I444.rgba")).expect("can't open");
//    println!("P6\n{width} {height}\n255\n", width=nx, height=ny);

    let lm = Lambertian::new(Vec3::new(1.0, 1.0, 1.0));
    let mut world = HitableList { list: Vec::new() };
    world.list.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, &lm)));
//    world.list.push(Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5)));
//    world.list.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let cam = Camera::new();
    
    for j in (0..ny).rev() {
//        println!("Line: {}", j);
        for i in 0..nx {
            let mut col = Vec3::zero();

            for s in 0..ns {
                let u = (i as f32 + rand()) / nx as f32;
                let v = (j as f32 +rand()) / ny as f32;
                
                let r = cam.get_ray(u,v);
                col += colour(&r, &world, 0);
            }

            col *= 1.0 / ns as f32;
            col = Vec3::new(col.r().sqrt(), col.g().sqrt(), col.g().sqrt());
            
            let ia  = [ (255.99 * col.r()) as u8, (255.99 * col.g()) as u8, (255.99 * col.b()) as u8, 255];
            file.write(&ia).expect("can't write");
        }
    }

    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(4.0, 5.0, 6.0);

    let mut v3 = v1 + v2;
    v3[1] = 10.0;

    v3 += 2.0 * Vec3::new(7.0, 8.0, 9.0);

    println!("!! {}", v3);

    let v4 = cross(Vec3::new(1.0,0.0,0.0), Vec3::new(0.0,1.0,0.0));
    
    println!("!! {}", v4);

    let r1 = Ray::new(v1,v2);
    println!("!! {}", r1);
}
