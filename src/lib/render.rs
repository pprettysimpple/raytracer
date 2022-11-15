use crate::entity::scene::Scene;
use crate::intersect::Intersect;
use crate::light::Light;
use crate::vec3::{cross_product, dot_product, Vec3};

#[derive(Debug, Copy, Clone)]
pub struct RenderConfig {
    pub width: usize,
    pub height: usize,
    pub fov: f32,
    pub origin: Vec3,
    pub view_dir: Vec3,
    pub background_color: Vec3,
    pub recursion_limit: usize,
    pub interest_point: Vec3,
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

fn cast_ray(
    config: &RenderConfig,
    cast_depth: usize,
    from: &Vec3,
    dir: &Vec3,
    scene: &Scene,
    lights: &[Light],
) -> Vec3 {
    if cast_depth >= config.recursion_limit {
        return config.background_color;
    }

    if let Some((hit, normal, material)) = scene.ray_intersect(from, dir) {
        let reflect_dir = reflect(dir, &normal).normalized();
        let refract_dir = refract(dir, &normal, material.refract_index).normalized();

        let reflect_color = cast_ray(config, cast_depth + 1, &hit, &reflect_dir, scene, lights);
        let refract_color = cast_ray(config, cast_depth + 1, &hit, &refract_dir, scene, lights);

        let (diffuse_light_intensity, specular_light_intensity) = lights
            .iter()
            .filter(|light| {
                let light_dir = (light.position - hit).normalized();
                if let Some((another_hit, _, _)) = scene.ray_intersect(&hit, &light_dir) {
                    hit.dist(&another_hit) > hit.dist(&light.position)
                } else {
                    true
                }
            })
            .map(|light| light.get_light_scales(&hit, dir, &normal, &material))
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
    config.background_color
}

pub fn render_scene_pixel(
    config: &RenderConfig,
    scene: &Scene,
    lights: &[Light],
    pixel: usize,
) -> Vec3 {
    let width_f = config.width as f32;
    let height_f = config.height as f32;

    let i = pixel / config.width;
    let j = pixel % config.width;

    let i_f = i as f32;
    let j_f = j as f32;

    let y_dir = Vec3::new(0.0, 1.0, 0.0);

    let i_vec = y_dir;
    let j_vec = cross_product(&config.view_dir, &y_dir).normalized();
    let width_real_f = config.view_dir.norm() * libm::sinf(config.fov / 2.0) * 2.0;
    let height_real_f = width_real_f * height_f / width_f;

    let j_step = j_vec * (width_real_f / width_f);
    let i_step = i_vec * (height_real_f / height_f);

    let dir = (config.view_dir
        + i_step * -2.0 * ((i_f - height_f / 2.0) as f32)
        + j_step * 2.0 * ((j_f - width_f / 2.0) as f32))
        .normalized();

    cast_ray(config, 0, &config.origin, &dir, scene, lights)
}
