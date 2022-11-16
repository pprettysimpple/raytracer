use alloc::vec::Vec;

use crate::entity::bounding_box::BoundingBox;
use crate::entity::triangle::Triangle;
use crate::intersect::Intersect;
use crate::material::Material;
use crate::ray::Ray;
use crate::render::RenderState;
use crate::utils::OrderedFloat32;
use crate::vec3::Vec3;

#[derive(Debug, Clone)]
pub struct Model {
    pub triangles: Vec<Triangle>,
    pub aabb: BoundingBox,
}

impl Model {
    pub fn from_faces(state: &RenderState, triangles: Vec<Triangle>) -> Model {
        let aabb = BoundingBox::new(
            triangles
                .iter()
                .flat_map(|triangle| {
                    triangle.points.iter().map(|vec_id| {
                        *state.vec_buf.load(*vec_id)
                    }).collect::<Vec<Vec3>>()
                })
                .collect::<Vec<Vec3>>()
                .as_slice(),
        );
        Model {
            triangles,
            aabb,
        }
    }
}

impl Intersect for Model {
    fn ray_intersect(&self, state: &RenderState, ray: Ray) -> Option<(Vec3, Vec3, Material)> {
        if !self.aabb.ray_check_intersect_standard(ray) {
            return None;
        }

        self.triangles
            .iter()
            .filter_map(|triangle| triangle.ray_intersect(state, ray))
            .min_by_key(|(hit, _, _)| OrderedFloat32::new(hit.dist_observer(&ray.from)))
    }
}
