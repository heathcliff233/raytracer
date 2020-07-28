use crate::{
    hittable::HitRecord,
    ray::Ray,
    rtweekend::{fmin, random_double},
    texture::{ConstTexture, Texture},
    vec3::{random_in_unit_sphere, random_unit_vector, reflect, refract, Color, Point3},
};
use std::sync::Arc;

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color;
}

pub struct Lambertian {
    pub albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(a: Color) -> Self {
        Self {
            albedo: Arc::new(ConstTexture { color_value: a }),
        }
    }
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
        *attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        true
    }
    fn emitted(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::zero()
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
    fn emitted(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::zero()
    }
}

pub struct Dielectric {
    pub ref_idx: f64,
}

impl Dielectric {
    pub fn new(r: f64) -> Self {
        Self { ref_idx: r }
    }
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
        if !rec.front_face {
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
        let reflect_prob = schlick(cos_theta, etai_over_etat);
        if random_double(0.0, 1.0) < reflect_prob {
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
    fn emitted(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::zero()
    }
}

pub fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

pub struct DiffuseLight {
    pub emit: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(c: Color) -> Self {
        Self {
            emit: Arc::new(ConstTexture { color_value: c }),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Color,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }
    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.emit.value(u, v, p)
    }
}
