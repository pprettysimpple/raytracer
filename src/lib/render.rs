use alloc::vec::Vec;
use crate::entity::scene::Scene;
use crate::intersect::Intersect;
use crate::light::Light;
use crate::material::Material;
use crate::ray::Ray;
use crate::utils::{EPSILON, MaterialBuf, MaterialIdx, Vec3Idx, VecBuf};
use crate::vec3::{cross_product, dot_product, Vec3};

#[derive(Debug, Clone)]
pub struct RenderState {
    pub width: usize,
    pub height: usize,
    pub fov: f32,
    pub origin: Vec3,
    pub view_dir: Vec3,
    pub background_color: Vec3,
    pub recursion_limit: usize,
    pub interest_point: Vec3,

    pub vec_buf: VecBuf,
    pub material_buf: MaterialBuf,

    pub scene: Scene,
    pub lights: Vec<Light>
}

impl RenderState {
    fn cast_ray(&self, cast_depth: usize, ray: Ray) -> Vec3 {
        if cast_depth >= self.recursion_limit {
            return self.background_color;
        }

        if let Some((hit, normal, material)) = self.scene.ray_intersect(self, ray) {
            let reflect_color = if libm::fabsf(material.albedo[2]) < EPSILON {
                Default::default()
            } else {
                let reflect_dir = reflect(&ray.dir, &normal).normalized();
                self.cast_ray(cast_depth + 1, Ray::new(hit, reflect_dir))
            };

            let refract_color = if libm::fabsf(material.albedo[3]) < EPSILON {
                Default::default()
            } else {
                let refract_dir = refract(&ray.dir, &normal, material.refract_index).normalized();
                self.cast_ray(cast_depth + 1, Ray::new(hit, refract_dir))
            };

            let (diffuse_light_intensity, specular_light_intensity) = self
                .lights
                .iter()
                .filter(|light| {
                    let light_dir = (light.position - hit).normalized();
                    if let Some((another_hit, _, _)) = self.scene.ray_intersect(self, Ray::new(hit, light_dir)) {
                        hit.dist_observer(&another_hit) > hit.dist_observer(&light.position)
                    } else {
                        true
                    }
                })
                .map(|light| light.get_light_scales(&hit, &ray.dir, &normal, &material))
                .fold((0.0, 0.0), |acc, val| (acc.0 + val.0, acc.1 + val.1));

            let albedo_it = material.albedo.iter();
            let scales = [
                material.diffuse_color * diffuse_light_intensity,
                Vec3::new(1.0, 1.0, 1.0) * specular_light_intensity,
                reflect_color,
                refract_color,
            ];

            return scales.iter().zip(albedo_it).map(|(a, b)| *a * *b).sum();
        }
        self.background_color
    }


    pub fn render_scene_pixel(
        &self,
        pixel_id: usize,
    ) -> Vec3 {
        let width_f = self.width as f32;
        let height_f = self.height as f32;

        let i = pixel_id / self.width;
        let j = pixel_id % self.width;

        let i_f = i as f32;
        let j_f = j as f32;

        let y_dir = Vec3::new(0.0, 1.0, 0.0);

        let i_vec = y_dir;
        let j_vec = cross_product(&self.view_dir, &y_dir).normalized();
        let width_real_f = self.view_dir.norm() * libm::sinf(self.fov / 2.0) * 2.0;
        let height_real_f = width_real_f * height_f / width_f;

        let j_step = j_vec * (width_real_f / width_f);
        let i_step = i_vec * (height_real_f / height_f);

        let dir = (self.view_dir
            + i_step * -2.0 * ((i_f - height_f / 2.0) as f32)
            + j_step * 2.0 * ((j_f - width_f / 2.0) as f32))
            .normalized();

        self.cast_ray(0, Ray::new(self.origin, dir))
    }

    pub fn push_vec(&mut self, vec: Vec3) -> Vec3Idx {
        self.vec_buf.push(vec)
    }

    pub fn push_material(&mut self, material: Material) -> MaterialIdx {
        self.material_buf.push(material)
    }
}

pub fn reflect(dir: &Vec3, normal: &Vec3) -> Vec3 {
    *dir - *normal * 2.0 * dot_product(dir, normal)
}

fn refract(dir: &Vec3, normal: &Vec3, eta_t: f32) -> Vec3 {
    refract_full(dir, normal, eta_t, 1.0)
}

fn refract_full(dir: &Vec3, normal: &Vec3, eta_t: f32, eta_i: f32) -> Vec3 {
    let cos_i = -(1.0_f32.min((-1.0_f32).max(dot_product(dir, normal))));

    if cos_i < 0.0 {
        return refract_full(dir, &-(*normal), eta_i, eta_t);
    }

    let eta = eta_i / eta_t;
    let k = 1.0 - eta * eta * (1.0 - cos_i * cos_i);

    if k < 0.0 {
        Vec3::new(1.0, 0.0, 0.0)
    } else {
        *dir * eta + *normal * (eta * cos_i - libm::sqrtf(k))
    }
}
