pub mod bounding_box;
pub mod model;
pub mod plane;
pub mod scene;
pub mod sphere;
pub mod triangle;

use crate::entity::model::Model;
use crate::entity::plane::Plane;
use crate::entity::scene::Scene;
use crate::entity::sphere::Sphere;
use crate::entity::triangle::Triangle;
use crate::intersect::Intersect;
use crate::material::Material;
use crate::vec3::Vec3;

pub enum Entity {
    Sphere(Sphere),
    Plane(Plane),
    Triangle(Triangle),
    Model(Model),
    Scene(Scene),
}

impl Intersect for Entity {
    fn ray_intersect(&self, from: &Vec3, dir: &Vec3) -> Option<(Vec3, Vec3, Material)> {
        match self {
            Entity::Sphere(obj) => obj.ray_intersect(from, dir),
            Entity::Plane(obj) => obj.ray_intersect(from, dir),
            Entity::Triangle(obj) => obj.ray_intersect(from, dir),
            Entity::Model(obj) => obj.ray_intersect(from, dir),
            Entity::Scene(obj) => obj.ray_intersect(from, dir),
        }
    }
}
