use crate::utils::OrderedFloat32;
use crate::vec3::Vec3;
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

    pub fn ray_check_intersect_standard(&self, from: &Vec3, dir: &Vec3) -> bool {
        let dir_inv = Vec3::new(dir.x.recip(), dir.y.recip(), dir.z.recip());
        let sign_x = (dir.x < 0.0) as usize;
        let sign_y = (dir.y < 0.0) as usize;
        let sign_z = (dir.z < 0.0) as usize;

        let mut ray_min = (self[sign_x].x - from.x) * dir_inv.x;
        let mut ray_max = (self[1 - sign_x].x - from.x) * dir_inv.x;

        let y_min = (self[sign_y].y - from.y) * dir_inv.y;
        let y_max = (self[1 - sign_y].y - from.y) * dir_inv.y;

        ray_min = max(ray_min, y_min);
        ray_max = min(ray_max, y_max);

        let z_min = (self[sign_z].z - from.z) * dir_inv.z;
        let z_max = (self[1 - sign_z].z - from.z) * dir_inv.z;

        ray_min = max(ray_min, z_min);
        ray_max = min(ray_max, z_max);

        max(ray_min, 0.0) <= ray_max
    }
}
