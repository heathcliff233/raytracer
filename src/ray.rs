use crate::vec3::{Color, Point3, Vec3};

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
