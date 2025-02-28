use crate::vec3::{Point3, Vec3};

pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray {
            orig: origin,
            dir: direction,
        }
    }

    pub fn origin(&self) -> &Point3 {
        &self.orig
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    // P(t) = O + tD, O is the origin, D is the direction, t is the parameter
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + (self.dir * t)
    }
}
