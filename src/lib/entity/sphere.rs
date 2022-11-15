use crate::intersect::Intersect;
use crate::material::Material;
use crate::utils::EPSILON;
use crate::vec3::{dot_product, Vec3};

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Vec3,
    radius: f32,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Material) -> Sphere {
        assert!(radius > 0.0);
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Intersect for Sphere {
    fn ray_intersect(&self, from: &Vec3, dir: &Vec3) -> Option<(Vec3, Vec3, Material)> {
        let l = self.center - *from;
        let tca = dot_product(&l, dir);
        let d2 = dot_product(&l, &l) - tca * tca;
        if d2 > self.radius * self.radius {
            return None;
        }
        let thc = libm::sqrtf(self.radius * self.radius - d2);
        let t1 = tca + thc;
        let mut t0 = tca - thc;
        if t0 < EPSILON {
            t0 = t1;
        }
        if t0 < EPSILON {
            return None;
        }
        let dist = t0;

        let hit = *from + *dir * dist;
        let normal = (hit - self.center).normalized();
        Some((hit, normal, self.material))
    }
}
