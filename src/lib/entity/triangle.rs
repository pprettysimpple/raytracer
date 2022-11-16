use crate::intersect::Intersect;
use crate::material::Material;
use crate::ray::Ray;
use crate::render::RenderState;
use crate::utils::{EPSILON, MaterialIdx, Vec3Idx};
use crate::vec3::{cross_product, dot_product, Vec3};

#[derive(Debug, Copy, Clone)]
pub struct Triangle {
    pub points: [Vec3Idx; 3],
    pub normal: Vec3,
    pub distance: f32,
    pub material: MaterialIdx,
}

impl Triangle {
    pub fn new(state: &RenderState, points: [Vec3Idx; 3], material: MaterialIdx) -> Triangle {
        let a = state.vec_buf.load(points[0]);
        let b = state.vec_buf.load(points[1]);
        let c = state.vec_buf.load(points[2]);

        let normal = cross_product(&(*b - *a), &(*c - *b)).normalized();
        let distance = dot_product(a, &normal);

        Triangle {
            points,
            normal,
            distance,
            material,
        }
    }
}

impl Intersect for Triangle {
    fn ray_intersect(&self, state: &RenderState, ray: Ray) -> Option<(Vec3, Vec3, Material)> {
        if dot_product(&ray.dir, &self.normal) > EPSILON {
            return None;
        }

        let a = state.vec_buf.load(self.points[0]);
        let b = state.vec_buf.load(self.points[1]);
        let c = state.vec_buf.load(self.points[2]);

        let a_to_b = *b - *a;
        let a_to_c = *c - *a;

        let u_vec = cross_product(&ray.dir, &a_to_c);

        let det = dot_product(&a_to_b, &u_vec);
        if libm::fabsf(det) < EPSILON {
            return None;
        }
        let normal = if det < EPSILON {
            -self.normal
        } else {
            self.normal
        };

        let inv_det = 1.0 / det;

        let a_to_origin = ray.from - *a;

        let u = dot_product(&a_to_origin, &u_vec) * inv_det;

        if !(0.0..=1.0).contains(&u) {
            return None;
        }

        let v_vec = cross_product(&a_to_origin, &a_to_b);

        let v = dot_product(&ray.dir, &v_vec) * inv_det;
        if v < -EPSILON || u + v > 1.0 + EPSILON {
            return None;
        }

        let dist = dot_product(&a_to_c, &v_vec) * inv_det;

        if dist > EPSILON {
            let self_material = state.material_buf.load(self.material);
            let hit = ray.from + ray.dir * dist;
            Some((hit, normal, *self_material))
        } else {
            None
        }
    }
}
