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

#[allow(dead_code)]
fn random_spheres(world: &mut HitableList, clear: Vec3, dist:f32) {
    for a in -11 .. 11 {
        for b in -11 .. 11 {
            let choose_mat = rand();
            let centre = Vec3::new(a as f32 + 0.9*rand(), 0.2, b as f32 + 0.9 * rand());
            if (centre - clear).length() > dist {
                if choose_mat < 0.7 {
                    // Diffuse
                    world.push(Box::new(Sphere::new(centre, 0.2,
                                                    Box::new(Lambertian::new(Vec3::new(rand()*rand(),
                                                                                       rand()*rand(),
                                                                                       rand()*rand()))))));
                } else if choose_mat < 0.85 {
                    // Metal
                    world.push(Box::new(Sphere::new(centre, 0.2,
                                                    Box::new(Metal::new(Vec3::new(0.5*(1.0+rand()),
                                                                                  0.5*(1.0+rand()),
                                                                                  0.5*(1.0+rand())),
                                                                        0.5*rand())))));
                } else {
                    // Glass
                    world.push(Box::new(Sphere::new(centre, 0.2,Box::new(Dielectric::new(1.5)))));
                }
            }
        }
    }
}

#[allow(dead_code)]
fn random_scene(aspect: f32) -> (HitableList, Camera) {

    let mut h = HitableList::new(Vec::new());
    
    h.push(Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Box::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))))));

    random_spheres(&mut h, Vec3::new(4.0, 0.2, 0.0), 0.9);
    
    h.push(Box::new(Sphere::new(Vec3::new(0.0,1.0,0.0), 1.0, Box::new(Dielectric::new(1.5)))));
    h.push(Box::new(Sphere::new(Vec3::new(-4.0,1.0,0.0), 1.0, Box::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))))));
    h.push(Box::new(Sphere::new(Vec3::new(4.0,1.0,0.0), 1.0, Box::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)))));

    let lookfrom = Vec3::new(14.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);

    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 0.05;
    
    let cam = Camera::new(lookfrom, lookat, Vec3::new(0.0,1.0,0.0), 20.0, aspect, aperture, dist_to_focus);

    (h,cam)
}


#[allow(dead_code)]
fn test_scene(aspect: f32) -> (HitableList, Camera) {
    let h = HitableList::new(vec!(
            Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Box::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5))))),
            Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Box::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))))),
            Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, Box::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.4)))),
            Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, Box::new(Dielectric::new(1.5)))),
            Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.45, Box::new(Dielectric::new(1.5))))
    ));

    let lookfrom = Vec3::new(14.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);

    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 0.2;
    
    let cam = Camera::new(lookfrom, lookat, Vec3::new(0.0,1.0,0.0), 20.0, aspect, aperture, dist_to_focus);

    (h,cam)
}

#[allow(dead_code)]
fn juggler_scene(aspect: f32) -> (HitableList, Camera) {
    let mut h = HitableList::new(vec!(
        // Ground
        Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Box::new(Lambertian::new(Vec3::new(0.5, 0.8, 0.5))))),

        // Juggler
        Box::new(Sphere::new(Vec3::new(-0.225, 1.325, -0.525), 0.150, Box::new(Metal::new(Vec3::new(0.90, 0.90, 0.90), 0.0)))),
        Box::new(Sphere::new(Vec3::new(-0.275, 1.475, 0.475), 0.150, Box::new(Metal::new(Vec3::new(0.90, 0.90, 0.90), 0.0)))),
        Box::new(Sphere::new(Vec3::new(-0.100, 1.700, -0.300), 0.150, Box::new(Metal::new(Vec3::new(0.90, 0.90, 0.90), 0.0)))),
        Box::new(Sphere::new(Vec3::new(0.000, 1.525, 0.000), 0.125, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(0.005, 1.530, 0.000), 0.125, Box::new(Metal::new(Vec3::new(0.20, 0.10, 0.10), 0.6)))),
        Box::new(Sphere::new(Vec3::new(-0.100, 1.525, 0.050), 0.037, Box::new(Metal::new(Vec3::new(0.10, 0.10, 1.00), 0.6)))),
        Box::new(Sphere::new(Vec3::new(-0.100, 1.525, -0.050), 0.037, Box::new(Metal::new(Vec3::new(0.10, 0.10, 1.00), 0.6)))),
        Box::new(Sphere::new(Vec3::new(0.000, 1.375, 0.000), 0.050, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(0.000, 1.150, 0.000), 0.200, Box::new(Metal::new(Vec3::new(1.00, 0.10, 0.10), 0.6)))),
        Box::new(Sphere::new(Vec3::new(0.000, 1.085, 0.000), 0.190, Box::new(Metal::new(Vec3::new(1.00, 0.10, 0.10), 0.6)))),
        Box::new(Sphere::new(Vec3::new(0.000, 1.020, 0.000), 0.180, Box::new(Metal::new(Vec3::new(1.00, 0.10, 0.10), 0.6)))),
        Box::new(Sphere::new(Vec3::new(0.000, 0.955, 0.000), 0.170, Box::new(Metal::new(Vec3::new(1.00, 0.10, 0.10), 0.6)))),
        Box::new(Sphere::new(Vec3::new(0.000, 0.890, 0.000), 0.160, Box::new(Metal::new(Vec3::new(1.00, 0.10, 0.10), 0.6)))),
        Box::new(Sphere::new(Vec3::new(0.000, 0.825, 0.000), 0.150, Box::new(Metal::new(Vec3::new(1.00, 0.10, 0.10), 0.6)))),
        Box::new(Sphere::new(Vec3::new(0.000, 0.725, 0.150), 0.050, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(-0.025, 0.671, 0.150), 0.050, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(-0.050, 0.617, 0.150), 0.050, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(-0.075, 0.562, 0.150), 0.050, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(-0.100, 0.508, 0.150), 0.050, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(-0.125, 0.454, 0.150), 0.050, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(-0.150, 0.400, 0.150), 0.050, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(-0.143, 0.343, 0.150), 0.046, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(-0.136, 0.286, 0.150), 0.043, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(-0.129, 0.229, 0.150), 0.039, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(-0.121, 0.171, 0.150), 0.036, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(-0.114, 0.114, 0.150), 0.032, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(-0.107, 0.057, 0.150), 0.029, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(-0.100, 0.000, 0.150), 0.025, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(0.000, 0.725, -0.150), 0.050, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(0.008, 0.671, -0.150), 0.050, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(0.017, 0.617, -0.150), 0.050, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(0.025, 0.562, -0.150), 0.050, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(0.033, 0.508, -0.150), 0.050, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(0.042, 0.454, -0.150), 0.050, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(0.050, 0.400, -0.150), 0.050, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(0.057, 0.343, -0.150), 0.046, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(0.064, 0.286, -0.150), 0.043, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(0.071, 0.229, -0.150), 0.039, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(0.079, 0.171, -0.150), 0.036, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(0.086, 0.114, -0.150), 0.032, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(0.093, 0.057, -0.150), 0.029, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(0.100, 0.000, -0.150), 0.025, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(0.000, 1.275, -0.175), 0.050, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(-0.008, 1.238, -0.196), 0.050, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(-0.017, 1.200, -0.217), 0.050, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(-0.025, 1.163, -0.237), 0.050, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(-0.033, 1.125, -0.258), 0.050, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(-0.042, 1.087, -0.279), 0.050, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(-0.050, 1.050, -0.300), 0.050, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(-0.082, 1.046, -0.329), 0.046, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(-0.114, 1.043, -0.357), 0.043, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(-0.146, 1.039, -0.386), 0.039, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(-0.179, 1.036, -0.414), 0.036, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(-0.211, 1.032, -0.443), 0.032, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(-0.243, 1.029, -0.471), 0.029, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(-0.275, 1.025, -0.500), 0.025, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(0.000, 1.275, 0.175), 0.050, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(-0.008, 1.238, 0.196), 0.050, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(-0.017, 1.200, 0.217), 0.050, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(-0.025, 1.163, 0.237), 0.050, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(-0.033, 1.125, 0.258), 0.050, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(-0.042, 1.087, 0.279), 0.050, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(-0.050, 1.050, 0.300), 0.050, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(-0.079, 1.071, 0.325), 0.046, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(-0.107, 1.093, 0.350), 0.043, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(-0.136, 1.114, 0.375), 0.039, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(-0.164, 1.136, 0.400), 0.036, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(-0.193, 1.157, 0.425), 0.032, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(-0.221, 1.179, 0.450), 0.029, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
        Box::new(Sphere::new(Vec3::new(-0.250, 1.200, 0.475), 0.025, Box::new(Metal::new(Vec3::new(1.00, 0.70, 0.70), 0.6)))),
    ));

    random_spheres(&mut h, Vec3::new(0.0, 0.2, 0.0), 1.1);
    
    let lookfrom = Vec3::new(-3.75, 1.375, 1.5);
    let lookat = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 0.1;
    
    let cam = Camera::new(lookfrom, lookat, Vec3::new(0.0,1.0,0.0), 30.0, aspect, aperture, dist_to_focus);

    (h, cam)
}

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

fn main() {
    let nx = 200;
    let ny = 150;
    let ns = 100;

    let (world, cam) = random_scene(nx as f32 / ny as f32);
    
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
            
            let ia  = [(255.99 * col.r()) as u8, (255.99 * col.g()) as u8, (255.99 * col.b()) as u8];
            file.write(&ia).expect("can't write");
        }
    }
}
