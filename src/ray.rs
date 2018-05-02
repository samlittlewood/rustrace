//
use std::fmt;

use vec3::*;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    a: Vec3,
    b: Vec3,
}

impl Ray {
    pub fn new(ia: Vec3, ib: Vec3) -> Ray { Ray { a:ia, b:ib } }
    pub fn origin(&self) -> Vec3 { self.a }
    pub fn direction(&self) -> Vec3 { self.b }
    pub fn point_at_parameter(&self, t: f32) -> Vec3 { self.a + t*self.b }
}

impl fmt::Display for Ray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ray(origin={}, directon={})", self.a, self.b)
    }
}
