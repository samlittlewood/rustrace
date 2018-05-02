
use std::f32::consts::PI;

use vec3::*;
use ray::*;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,

    u:Vec3,
    v:Vec3,
    _w:Vec3,

    lens_radius:f32
}

impl Camera {
    pub fn new(lookfrom:Vec3, lookat:Vec3, vup: Vec3, vfov: f32, aspect:f32, aperture:f32, focus:f32) -> Camera {
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
            u: u, v: v, _w: w,
            lens_radius: aperture/2.0
        }
    }

    pub fn get_ray(&self, u: f32, v:f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(self.origin + offset, self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin - offset) }
}

