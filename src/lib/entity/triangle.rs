use crate::intersect::Intersect;
use crate::material::Material;
use crate::utils::EPSILON;
use crate::vec3::{cross_product, dot_product, Vec3};

#[derive()]
pub struct Triangle {
    pub points: [Vec3; 3],
    pub normal: Vec3,
    pub distance: f32,
    pub material: Material,
}

impl Triangle {
    pub fn new(points: [Vec3; 3], material: Material) -> Triangle {
        let normal = cross_product(&(points[1] - points[0]), &(points[2] - points[1]));
        Triangle {
            points,
            normal: normal.normalized(),
            distance: dot_product(&points[0], &normal),
            material,
        }
    }
}

impl Intersect for Triangle {
    fn ray_intersect(&self, from: &Vec3, dir: &Vec3) -> Option<(Vec3, Vec3, Material)> {
        let a = &self.points[0];
        let b = &self.points[1];
        let c = &self.points[2];

        if dot_product(dir, &self.normal) > EPSILON {
            return None;
        }

        let a_to_b = *b - *a;
        let a_to_c = *c - *a;

        let u_vec = cross_product(dir, &a_to_c);

        let det = dot_product(&a_to_b, &u_vec);
        if det < EPSILON {
            return None;
        }
        let normal = if det < EPSILON {
            -self.normal
        } else {
            self.normal
        };

        let inv_det = 1.0 / det;

        let a_to_origin = *from - *a;

        let u = dot_product(&a_to_origin, &u_vec) * inv_det;

        if !(0.0..=1.0).contains(&u) {
            return None;
        }

        let v_vec = cross_product(&a_to_origin, &a_to_b);

        let v = dot_product(dir, &v_vec) * inv_det;
        if v < -EPSILON || u + v > 1.0 + EPSILON {
            return None;
        }

        let dist = dot_product(&a_to_c, &v_vec) * inv_det;

        if dist > EPSILON {
            let hit = *from + *dir * dist;
            Some((hit, normal, self.material))
        } else {
            None
        }
    }
}
