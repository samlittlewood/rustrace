
use vec3::*;
use ray::*;

#[derive(Copy, Clone)]
pub struct Hit<'a> {
    pub t: f32,
    pub p: Vec3,
    pub n: Vec3,
    pub m: &'a Box<Material +'a>,
}

impl<'a> Hit<'a> {
    pub fn new(nt: f32, np: Vec3, nn: Vec3, mm: &'a Box<Material+'a>) -> Hit<'a> { Hit { t:nt, p:np, n:nn, m:mm } }
}

pub struct Scattered {
    pub scattered: Ray,
    pub attenuation: Vec3,
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &Hit) -> Option<Scattered>;
}

//  Lambertian
//
pub struct Lambertian {
    albedo: Vec3
}

impl Lambertian {
    pub fn new(a : Vec3) -> Lambertian { Lambertian { albedo: a } }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &Hit, ) -> Option<Scattered> {
        let target = rec.p + rec.n + random_in_unit_sphere();
        Some(Scattered { scattered:Ray::new(rec.p, target - rec.p), attenuation:self.albedo })
    }
}


// Metal
//
pub struct Metal {
    albedo: Vec3,
    fuzz: f32
}

impl Metal {
    pub fn new(a : Vec3, f: f32) -> Metal { Metal { albedo:a, fuzz:f } }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &Hit, ) -> Option<Scattered> {
        let s = Ray::new(rec.p, reflect(unit_vector(r_in.direction()), rec.n) + self.fuzz * random_in_unit_sphere());

        if dot(s.direction(), rec.n) > 0.0 {
            Some(Scattered { scattered:s, attenuation:self.albedo })
        } else {
            None
        }
    }
}

// Dielectric
//
pub struct Dielectric {
    ior: f32
}

impl Dielectric {
    pub fn new(i: f32) -> Dielectric { Dielectric { ior:i } }
}

fn schlick(cosine: f32, ior: f32) -> f32 {
    let r0 = (1.0-ior) / (1.0+ior);
    let r0r0 = r0*r0;
    r0r0 + (1.0-r0r0) * (1.0-cosine).powf(5.0)
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &Hit, ) -> Option<Scattered> {
        let attn = Vec3::new(1.0, 1.0, 1.0);
        let reflected = reflect(r_in.direction(), rec.n);
        
        let (outward_normal, ni_over_nt, cosine) = if dot(r_in.direction(), rec.n) > 0.0 {
            (-rec.n, self.ior, self.ior * dot(r_in.direction(), rec.n) / r_in.direction().length())
        } else {
            (rec.n, 1.0 / self.ior, -dot(r_in.direction(), rec.n) / r_in.direction().length())
        };

        match refract(r_in.direction(), outward_normal, ni_over_nt) {
            Some(refracted) => {
                if rand() >= schlick(cosine, self.ior) {
                    Some(Scattered { scattered:Ray::new(rec.p, refracted), attenuation: attn})
                } else {
                    Some(Scattered { scattered:Ray::new(rec.p, reflected), attenuation: attn})
                }
            }
            None => {
                Some(Scattered { scattered:Ray::new(rec.p, reflected), attenuation: attn})
            }
        }
    }
}

