use crate::entity::Entity;
use crate::intersect::Intersect;
use crate::material::Material;
use crate::utils::OrderedFloat32;
use crate::vec3::Vec3;
use alloc::boxed::Box;

#[derive()]
pub struct Scene {
    pub entities: Box<[Entity]>,
}

impl Scene {
    pub fn new(entities: Box<[Entity]>) -> Scene {
        Scene { entities }
    }
}

impl Intersect for Scene {
    fn ray_intersect(&self, from: &Vec3, dir: &Vec3) -> Option<(Vec3, Vec3, Material)> {
        self.entities
            .iter()
            .filter_map(|entity| entity.ray_intersect(from, dir))
            .min_by_key(|(hit, _, _)| OrderedFloat32::new(hit.dist_observer(from)))
    }
}
