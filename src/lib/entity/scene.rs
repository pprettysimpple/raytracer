use crate::entity::Entity;
use crate::intersect::Intersect;
use crate::material::Material;
use crate::utils::OrderedFloat32;
use crate::vec3::Vec3;
use alloc::boxed::Box;
use crate::ray::Ray;
use crate::render::RenderState;

#[derive(Debug, Clone)]
pub struct Scene {
    pub entities: Box<[Entity]>,
}

impl Scene {
    pub fn new(entities: Box<[Entity]>) -> Scene {
        Scene { entities }
    }
}

impl Intersect for Scene {
    fn ray_intersect(&self, state: &RenderState, ray: Ray) -> Option<(Vec3, Vec3, Material)> {
        self.entities
            .iter()
            .filter_map(|entity| entity.ray_intersect(state, ray))
            .min_by_key(|(hit, _, _)| OrderedFloat32::new(hit.dist_observer(&ray.from)))
    }
}
