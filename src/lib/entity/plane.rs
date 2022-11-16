use crate::intersect::Intersect;
use crate::material::Material;
use crate::ray::Ray;
use crate::render::RenderState;
use crate::utils::EPSILON;
use crate::vec3::{dot_product, Vec3};

#[derive(Debug, Copy, Clone)]
pub struct Plane {
    pub point: Vec3,
    pub normal: Vec3,
    material: Material,
}

impl Plane {
    pub fn new(point: Vec3, normal: Vec3, material: Material) -> Plane {
        Plane {
            point,
            normal,
            material,
        }
    }
}

impl Intersect for Plane {
    fn ray_intersect(&self, _state: &RenderState, ray: Ray) -> Option<(Vec3, Vec3, Material)> {
        let denominator = dot_product(&ray.dir, &self.normal);
        if denominator > -EPSILON {
            return None;
        }
        let numerator = dot_product(&(self.point - ray.from), &self.normal);
        if numerator * denominator < EPSILON {
            return None;
        }

        let hit = ray.from + ray.dir * (numerator / denominator);
        Some((hit, self.normal, self.material))
    }
}
