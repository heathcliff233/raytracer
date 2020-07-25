use crate::{
    hittable::HitRecord,
    ray::Ray,
    rtweekend::fmin,
    vec3::{random_in_unit_sphere, random_unit_vector, reflect, refract, Color},
};
use std::cmp::min;

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let scatter_direction = rec.normal + random_unit_vector();
        *scattered = Ray {
            orig: rec.p,
            dir: scatter_direction,
        };
        *attenuation = self.albedo;
        true
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(a: &Color, f: f64) -> Self {
        if f < 1.0 {
            return Self {
                albedo: *a,
                fuzz: f,
            };
        }
        Self {
            albedo: *a,
            fuzz: 1.0,
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(&r_in.dir.unit(), &rec.normal);
        *scattered = Ray {
            orig: rec.p,
            dir: reflected + random_in_unit_sphere() * self.fuzz,
        };
        *attenuation = self.albedo;
        scattered.dir * rec.normal > 0.0
    }
}

pub struct Dielectric {
    pub ref_idx: f64,
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::ones();
        let mut etai_over_etat = 1.0 / self.ref_idx;
        if rec.front_face == false {
            etai_over_etat = self.ref_idx;
        }
        let unit_dir = r_in.dir.unit();
        let cos_theta = fmin(1.0, -unit_dir * rec.normal);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        if etai_over_etat * sin_theta > 1.0 {
            let reflected = reflect(&unit_dir, &rec.normal);
            *scattered = Ray {
                orig: rec.p,
                dir: reflected,
            };
            return true;
        }
        let refracted = refract(&unit_dir, &rec.normal, etai_over_etat);
        *scattered = Ray {
            orig: rec.p,
            dir: refracted,
        };
        true
    }
}
