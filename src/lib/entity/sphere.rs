use crate::intersect::Intersect;
use crate::material::Material;
use crate::ray::Ray;
use crate::render::RenderState;
use crate::utils::EPSILON;
use crate::vec3::{dot_product, Vec3};

#[derive(Debug, Copy, Clone)]
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
    fn ray_intersect(&self, _state: &RenderState, ray: Ray) -> Option<(Vec3, Vec3, Material)> {
        let l = self.center - ray.from;
        let tca = dot_product(&l, &ray.dir);
        let d2 = dot_product(&l, &l) - tca * tca;
        let self_radius_sq = self.radius * self.radius;
        if d2 > self_radius_sq {
            return None;
        }
        let thc = libm::sqrtf(self_radius_sq - d2);
        let t1 = tca + thc;
        let mut t0 = tca - thc;
        if t0 < EPSILON {
            t0 = t1;
        }
        if t0 < EPSILON {
            return None;
        }
        let dist = t0;

        let hit = ray.from + ray.dir * dist;
        let normal = (hit - self.center).normalized();
        Some((hit, normal, self.material))
    }
}
