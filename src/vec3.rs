extern crate rand;

use std::fmt;
use std::ops::*;
use rand::Rng;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    e: [f32; 3]
}

impl Vec3 {
    pub fn zero() -> Vec3 { Vec3 { e : [ 0.0, 0.0, 0.0] } }
    pub fn new(e0: f32, e1: f32, e2: f32) -> Vec3 { Vec3 { e : [ e0, e1, e2] } }
    pub fn x(&self) -> f32 { self.e[0] }
    pub fn y(&self) -> f32 { self.e[1] }
    pub fn z(&self) -> f32 { self.e[2] }
    
    pub fn r(&self) -> f32 { self.e[0] }
    pub fn g(&self) -> f32 { self.e[1] }
    pub fn b(&self) -> f32 { self.e[2] }

    pub fn length(&self) -> f32 { (self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2]).sqrt() }
    pub fn length_squared(&self) -> f32 { self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2] }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.e[0], self.e[1], self.e[2])
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 { Vec3 { e: [ -self.e[0], -self.e[1], -self.e[2]] } }
}

impl Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, i:usize ) -> &f32 { &self.e[i] }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i:usize ) -> &mut f32 { &mut self.e[i] }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 { Vec3 { e: [ self.e[0] + other.e[0], self.e[1] + other.e[1], self.e[2] + other.e[2]] } }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 { Vec3 { e: [ self.e[0] - other.e[0], self.e[1] - other.e[1], self.e[2] - other.e[2]] } }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 { Vec3 { e: [ self.e[0] * other.e[0], self.e[1] * other.e[1], self.e[2] * other.e[2]] } }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: f32) -> Vec3 { Vec3 { e: [ self.e[0] * other, self.e[1] * other, self.e[2] * other] } }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 { Vec3 { e: [ self * other.e[0], self * other.e[1], self * other.e[2]] } }
}

impl Div for Vec3 {
    type Output = Vec3;
    fn div(self, other: Vec3) -> Vec3 { Vec3 { e: [ self.e[0] / other.e[0], self.e[1] / other.e[1], self.e[2] / other.e[2]] } }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, other: f32) -> Vec3 { Vec3 { e: [ self.e[0] / other, self.e[1] / other, self.e[2] / other] } }
}

impl Div<Vec3> for f32 {
    type Output = Vec3;
    fn div(self, other: Vec3) -> Vec3 { Vec3 { e: [ self / other.e[0], self / other.e[1], self / other.e[2]] } }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = Vec3 { e: [ self.e[0] + other.e[0], self.e[1] + other.e[1], self.e[2] + other.e[2]] }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        *self = Vec3 { e: [ self.e[0] - other.e[0], self.e[1] - other.e[1], self.e[2] - other.e[2]] }
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        *self = Vec3 { e: [ self.e[0] * other.e[0], self.e[1] * other.e[1], self.e[2] * other.e[2]] }
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, other: f32) {
        *self = Vec3 { e: [ self.e[0] * other, self.e[1] * other, self.e[2] * other] }
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Vec3) {
        *self = Vec3 { e: [ self.e[0] / other.e[0], self.e[1] / other.e[1], self.e[2] / other.e[2]] }
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, other: f32) {
        *self = Vec3 { e: [ self.e[0] / other, self.e[1] / other, self.e[2] / other] }
    }
}

pub fn dot(v1: Vec3, v2: Vec3) -> f32 {
    v1.e[0]*v2.e[0] + v1.e[1]*v2.e[1] + v1.e[2]*v2.e[2]
}

pub fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
    Vec3 { e:[v1.e[1] * v2.e[2] - v1.e[2] * v2.e[1],
              -(v1.e[0] * v2.e[2] - v1.e[2] * v2.e[0]),
              v1.e[0] * v2.e[1] - v1.e[1] * v2.e[0]] }
}


pub fn unit_vector(v: Vec3) -> Vec3  { v * (1.0 / v.length()) }

pub fn reflect(v: Vec3, n:Vec3) -> Vec3 { v - 2.8 * dot(v,n) * n }

pub fn refract(v: Vec3, n:Vec3, ni_over_nt:f32) -> Option<Vec3> {
    let uv = unit_vector(v);
    let dt = dot(uv, n);
    let discriminant = 1.0 - ni_over_nt*ni_over_nt*(1.0 - dt*dt);
    if discriminant > 0.0 {
        Some(ni_over_nt*(uv - n*dt) - n*(discriminant.sqrt()))
    } else {
        None
    }
}

pub fn rand() -> f32 {
    rand::thread_rng().gen_range(0.0, 1.0)
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = (2.0 * Vec3::new(rand(), rand(), rand())) - Vec3::new(1.0,1.0,1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = (2.0 * Vec3::new(rand(), rand(), 0.0)) - Vec3::new(1.0,1.0,0.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}
