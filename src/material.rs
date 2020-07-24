use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec3::{random_unit_vector, reflect, Color},
};

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
            dir: reflected,
        };
        *attenuation = self.albedo;
        scattered.dir * rec.normal > 0.0
    }
}
