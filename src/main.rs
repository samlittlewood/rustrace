//
extern crate rand;

pub mod vec3;
pub mod ray;
pub mod material;
pub mod hitable;
pub mod camera;

use std::io::Write;
use std::fs::File;
use std::path::Path;
use std::f32;

use vec3::*;
use ray::*;
use material::*;
use hitable::*;
use camera::*;

fn colour(r: &Ray, world: &Hitable, depth: i32) -> Vec3 {
        
    match world.hit(r, 0.001, f32::MAX) {
        Some(rec) => {
            if depth > 50 {
                Vec3::zero()
            } else {
                match rec.m.scatter(r, &rec) {
                    Some(s) => s.attenuation * colour(&s.scattered, world, depth+1),
                    None =>  Vec3::zero()
                }
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

#[allow(dead_code)]
fn random_scene() -> HitableList {

    let mut world = HitableList::new(Vec::new());
    
    world.push(Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Box::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))))));

    for a in -11 .. 11 {
        for b in -11 .. 11 {
            let choose_mat = rand();
            let centre = Vec3::new(a as f32 + 0.9*rand(), 0.2, b as f32 + 0.9 * rand());
            if (centre - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    world.push(Box::new(Sphere::new(centre, 0.2,
                                                         Box::new(Lambertian::new(Vec3::new(rand()*rand(), rand()*rand(), rand()*rand()))))));
                } else if choose_mat < 0.95 {
                    // Metal
                    world.push(Box::new(Sphere::new(centre, 0.2,
                                                         Box::new(Metal::new(Vec3::new(0.5*(1.0+rand()), 0.5*(1.0+rand()), 0.5*(1.0+rand())), 0.5*rand())))));
                                                          
                } else {
                    // Glass
                    world.push(Box::new(Sphere::new(centre, 0.2,Box::new(Dielectric::new(1.5)))));
                }
                   
                    
            }
        }
    }
    world.push(Box::new(Sphere::new(Vec3::new(0.0,1.0,0.0), 1.0, Box::new(Dielectric::new(1.5)))));
    world.push(Box::new(Sphere::new(Vec3::new(-4.0,1.0,0.0), 1.0, Box::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))))));
    world.push(Box::new(Sphere::new(Vec3::new(4.0,1.0,0.0), 1.0, Box::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)))));

    world
}


#[allow(dead_code)]
fn test_scene() -> HitableList {
    HitableList::new(vec!(
            Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Box::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5))))),
            Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Box::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))))),
            Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, Box::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.4)))),
            Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, Box::new(Dielectric::new(1.5)))),
            Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.45, Box::new(Dielectric::new(1.5))))
        ))
}

fn main() {
    let nx = 200;
    let ny = 150;
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
