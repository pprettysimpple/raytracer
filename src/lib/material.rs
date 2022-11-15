use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub refract_index: f32,
    pub albedo: [f32; 4],
    pub diffuse_color: Vec3,
    pub spectacular_exp: f32,
}

impl Material {
    pub fn new(
        refract_index: f32,
        albedo: [f32; 4],
        diffuse_color: Vec3,
        spectacular_exp: f32,
    ) -> Material {
        Material {
            refract_index,
            albedo,
            diffuse_color,
            spectacular_exp,
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Material {
            refract_index: 1.0,
            albedo: [1.0, 0.0, 0.0, 0.0],
            diffuse_color: Default::default(),
            spectacular_exp: 0.0,
        }
    }
}
