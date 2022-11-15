use libm::powf;

use crate::material::Material;
use crate::render::reflect;
use crate::vec3::{dot_product, Vec3};

#[derive(Debug, Copy, Clone)]
pub struct Light {
    pub position: Vec3,
    pub intensity: f32,
}

impl Light {
    pub fn new(position: Vec3, intensity: f32) -> Light {
        Light {
            position,
            intensity,
        }
    }

    pub fn get_light_scales(
        &self,
        hit: &Vec3,
        dir: &Vec3,
        normal: &Vec3,
        material: &Material,
    ) -> (f32, f32) {
        let light_direction = (self.position - *hit).normalized();
        let scale_light = dot_product(&light_direction, normal);

        let diffuse_light_intensity = self.intensity * 0.0_f32.max(scale_light);
        let specular_light_intensity = {
            let reflected = -reflect(&-light_direction, normal);
            let base = dot_product(&reflected, dir);
            powf(base.max(0.0_f32), material.spectacular_exp) * self.intensity
        };

        (diffuse_light_intensity, specular_light_intensity)
    }
}
