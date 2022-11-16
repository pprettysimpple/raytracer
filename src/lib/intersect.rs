use crate::ray::Ray;
use crate::render::RenderState;
use super::material::Material;
use super::vec3::Vec3;

pub trait Intersect {
    fn ray_intersect(&self, state: &RenderState, ray: Ray) -> Option<(Vec3, Vec3, Material)>;
}
