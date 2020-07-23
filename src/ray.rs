use crate::vec3::{Color, Point3, Vec3};

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> bool {
    let oc = r.orig - *center;
    let a = r.dir * r.dir;
    let b = oc * r.dir * 2.0;
    let c = oc * oc - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Vec3 {
        self.orig + self.dir * t
    }

    pub fn ray_color(&self) -> Color {
        if hit_sphere(
            &Point3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            0.5,
            &self,
        ) {
            return Color {
                x: 255.0,
                y: 0.0,
                z: 0.0,
            };
        }
        let dirc: Vec3 = self.dir.unit();
        let t = 0.5 * (dirc.y + 1.0);
        Color::ones() * 255.0 * (1.0 - t)
            + Color {
                x: 0.5,
                y: 0.7,
                z: 1.0,
            } * t
                * 255.0
    }
}
