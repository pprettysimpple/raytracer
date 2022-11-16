use crate::utils::OrderedFloat32;
use crate::vec3::Vec3;
use crate::ray::Ray;
use core::ops::Index;

#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    min: Vec3,
    max: Vec3,
}

impl Index<usize> for BoundingBox {
    type Output = Vec3;

    fn index(&self, index: usize) -> &Self::Output {
        if index == 0 {
            &self.min
        } else {
            &self.max
        }
    }
}

#[inline(always)]
fn min(x: f32, y: f32) -> f32 {
    if x < y {
        x
    } else {
        y
    }
}

#[inline(always)]
fn max(x: f32, y: f32) -> f32 {
    if x > y {
        x
    } else {
        y
    }
}

impl BoundingBox {
    pub fn new(points: &[Vec3]) -> BoundingBox {
        BoundingBox {
            min: Vec3::new(
                points
                    .iter()
                    .min_by_key(|vec| OrderedFloat32::new(vec.x))
                    .unwrap()
                    .x,
                points
                    .iter()
                    .min_by_key(|vec| OrderedFloat32::new(vec.y))
                    .unwrap()
                    .y,
                points
                    .iter()
                    .min_by_key(|vec| OrderedFloat32::new(vec.z))
                    .unwrap()
                    .z,
            ),
            max: Vec3::new(
                points
                    .iter()
                    .max_by_key(|vec| OrderedFloat32::new(vec.x))
                    .unwrap()
                    .x,
                points
                    .iter()
                    .max_by_key(|vec| OrderedFloat32::new(vec.y))
                    .unwrap()
                    .y,
                points
                    .iter()
                    .max_by_key(|vec| OrderedFloat32::new(vec.z))
                    .unwrap()
                    .z,
            ),
        }
    }

    pub fn ray_check_intersect_standard(&self, ray: Ray) -> bool {
        let mut ray_min = (self[ray.sign_x].x - ray.from.x) * ray.inv_dir.x;
        let mut ray_max = (self[1 - ray.sign_x].x - ray.from.x) * ray.inv_dir.x;

        let y_min = (self[ray.sign_y].y - ray.from.y) * ray.inv_dir.y;
        let y_max = (self[1 - ray.sign_y].y - ray.from.y) * ray.inv_dir.y;

        ray_min = max(ray_min, y_min);
        ray_max = min(ray_max, y_max);

        let z_min = (self[ray.sign_z].z - ray.from.z) * ray.inv_dir.z;
        let z_max = (self[1 - ray.sign_z].z - ray.from.z) * ray.inv_dir.z;

        ray_min = max(ray_min, z_min);
        ray_max = min(ray_max, z_max);

        max(ray_min, 0.0) <= ray_max
    }
}
