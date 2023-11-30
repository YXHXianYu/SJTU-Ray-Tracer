use crate::vec3::Vec3;

pub struct Ray {
    ori: Vec3,
    dir: Vec3,
}

#[allow(dead_code)]
impl Ray {
    pub fn new() -> Ray {
        Ray { ori: Vec3::new(), dir: Vec3::new() }
    }
    pub fn from(ori: Vec3, dir: Vec3) -> Ray {
        Ray { ori, dir }
    }

    pub fn origin(&self) -> Vec3 {
        self.ori
    }
    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.ori + self.dir * t
    }
}
