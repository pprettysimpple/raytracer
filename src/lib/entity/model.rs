use alloc::vec::Vec;

use crate::entity::bounding_box::BoundingBox;
use crate::entity::scene::Scene;
use crate::entity::triangle::Triangle;
use crate::entity::Entity;
use crate::intersect::Intersect;
use crate::material::Material;
use crate::vec3::Vec3;

pub struct Model {
    pub scene: Scene,
    pub aabb: BoundingBox,
}

impl Model {
    pub fn from_faces(faces: Vec<Triangle>) -> Model {
        let aabb = BoundingBox::new(
            faces
                .iter()
                .flat_map(|triangle| triangle.points)
                .collect::<Vec<Vec3>>()
                .as_slice(),
        );
        Model {
            scene: Scene::new(
                faces
                    .into_iter()
                    .map(Entity::Triangle)
                    .collect::<Vec<Entity>>()
                    .into_boxed_slice(),
            ),
            aabb,
        }
    }
}

impl Intersect for Model {
    fn ray_intersect(&self, from: &Vec3, dir: &Vec3) -> Option<(Vec3, Vec3, Material)> {
        if !self.aabb.ray_check_intersect_standard(from, dir) {
            return None;
        }

        self.scene.ray_intersect(from, dir)
    }
}
