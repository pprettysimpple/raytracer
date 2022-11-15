use std::borrow::Borrow;
use std::fs::read_to_string;
use std::str::FromStr;

use raytracer::entity::model::Model;
use raytracer::entity::plane::Plane;
use raytracer::entity::scene::Scene;
use raytracer::entity::sphere::Sphere;
use raytracer::entity::triangle::Triangle;
use raytracer::entity::Entity;
use raytracer::light::Light;
use raytracer::material::Material;
use raytracer::render::{render_scene_pixel, RenderConfig};
use raytracer::vec3::Vec3;

use image::{ImageBuffer, Rgb, RgbImage};
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;
use show_image::{run_context, ImageInfo, ImageView};

fn load_model_faces(
    filename: &String,
    base_material: Material,
    scale: f32,
    offset: &Vec3,
) -> Vec<Triangle> {
    if !filename.ends_with(".obj") {
        panic!("File format is bad, expected *.obj, but found {filename}");
    }

    let content = read_to_string(filename.clone())
        .unwrap_or_else(|_| panic!("Failed to load model from file: {filename}"));
    let content_lines = content.split('\n');

    let mut points = content_lines
        .clone()
        .filter_map(|line| {
            if !line.starts_with("v ") {
                return None;
            }
            let stripped = line.strip_prefix("v ")?;
            let points: Vec<f32> = stripped
                .split(' ')
                .filter_map(|x| f32::from_str(x).ok())
                .take(3)
                .collect();
            Some(Vec3::new(points[0], points[1], points[2]))
        })
        .collect::<Vec<Vec3>>();

    points
        .iter_mut()
        .for_each(|val| *val = *val * scale + *offset);

    content_lines
        .filter_map(|line| {
            if !line.starts_with("f ") {
                return None;
            }
            let stripped = line.strip_prefix("f ")?;

            let point_indexes: Vec<i32> = stripped
                .split(' ')
                .filter_map(|str| {
                    i32::from_str(
                        str.chars()
                            .take_while(char::is_ascii_digit)
                            .collect::<String>()
                            .as_str(),
                    )
                    .ok()
                })
                .collect();

            Some(point_indexes)
        })
        .map(|vec| -> [i32; 3] {
            vec.try_into()
                .expect("Failed to parse, only triangles are supported")
        })
        .filter_map(move |vec| {
            if !vec
                .iter()
                .all(|val| points.get((val - 1) as usize).is_some())
            {
                return None;
            }
            Some(Triangle::new(
                vec.map(|val| (points[(val - 1) as usize])),
                base_material,
            ))
        })
        .collect::<Vec<Triangle>>()
}

fn show_frame_buffer(
    config: &RenderConfig,
    frame_buffer: &[Vec3],
) -> Option<ImageBuffer<Rgb<u8>, Vec<u8>>> {
    let to_color = |color: f32| (color.min(1.0).max(0.0) * 255.0) as u8;
    RgbImage::from_vec(
        config.width as u32,
        config.height as u32,
        frame_buffer
            .iter()
            .map(|v| [v.x, v.y, v.z].map(to_color))
            .collect::<Vec<[u8; 3]>>()
            .concat(),
    )
}

fn main() {
    let mut config = RenderConfig {
        width: 400,
        height: 300,
        fov: 0.95,
        origin: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        view_dir: Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        interest_point: Vec3 {
            x: 0.0,
            y: 0.0,
            z: -17.0,
        },
        background_color: Vec3 {
            x: 0.2,
            y: 0.7,
            z: 0.8,
        },
        recursion_limit: 4,
    };

    let ivory = Material::new(1.0, [0.6, 0.3, 0.1, 0.0], Vec3::new(0.4, 0.4, 0.3), 50.0);
    let glass = Material::new(1.5, [0.0, 0.5, 0.1, 0.8], Vec3::new(0.6, 0.7, 0.8), 125.0);
    let red_rubber = Material::new(1.0, [0.9, 0.1, 0.0, 0.0], Vec3::new(0.3, 0.1, 0.1), 10.0);
    let mirror = Material::new(1.0, [0.0, 10.0, 0.8, 0.0], Vec3::new(1.0, 1.0, 1.0), 1425.0);
    // let blue_rubber = Material::new(1.0, [0.9, 0.1, 0.0, 0.0], Vec3::new(0.1, 0.1, 0.3), 10.0);

    let spheres = [
        Sphere::new(Vec3::new(-3.0, 0.0, -16.0), 2.0, ivory),
        Sphere::new(Vec3::new(-1.0, -1.5, -12.0), 2.0, glass),
        Sphere::new(Vec3::new(1.5, -0.5, -18.0), 3.0, red_rubber),
        Sphere::new(Vec3::new(7.0, 5.0, -18.0), 4.0, mirror),
    ];

    let planes = [
        Plane::new(Vec3::new(0.0, -4.0, 0.0), Vec3::new(0.0, 1.0, 0.0), ivory),
        Plane::new(
            Vec3::new(0.0, 60.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            red_rubber,
        ),
        Plane::new(Vec3::new(0.0, 0.0, -60.0), Vec3::new(0.0, 0.0, 1.0), mirror),
        Plane::new(Vec3::new(0.0, 0.0, 60.0), Vec3::new(0.0, 0.0, -1.0), mirror),
        Plane::new(Vec3::new(35.0, 0.0, 0.0), Vec3::new(-1.0, 0.0, 0.0), mirror),
        Plane::new(Vec3::new(-35.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), mirror),
    ];

    // only simple models supported:
    // - only triangles
    // - points order will specify the normal for triangle
    let models = [Model::from_faces(load_model_faces(
        &String::from("data/duck.obj"),
        mirror,
        1.0,
        &Default::default(),
    ))];

    let lights = [
        Light::new(Vec3::new(-20.0, 20.0, 20.0), 1.5),
        Light::new(Vec3::new(30.0, 50.0, -25.0), 1.8),
        Light::new(Vec3::new(30.0, 20.0, 30.0), 1.7),
        Light::new(Vec3::new(-20.0, -20.0, -30.0), 10.0),
    ];

    let scene = Scene::new(
        models
            .into_iter()
            .map(Entity::Model)
            .chain(spheres.into_iter().map(Entity::Sphere))
            .chain(planes.into_iter().map(Entity::Plane))
            .collect::<Vec<Entity>>()
            .into_boxed_slice(),
    );

    println!("Entities to render: {}", scene.entities.len());

    let len = (config.width * config.height) as usize;
    let mut frame_buffer: Vec<Vec3> = vec![
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        len
    ];

    run_context(move || {
        let window = show_image::create_window("image", Default::default())
            .expect("Failed to create window");

        let mut frame_count = 0;

        loop {
            frame_count += 1;
            let t = (frame_count as f32) / 20.0;
            config.view_dir = Vec3::new(t.cos(), 0.0, t.sin()).normalized();
            config.origin = config.interest_point - config.view_dir * 15.0;

            frame_buffer
                .par_iter_mut()
                .enumerate()
                .for_each(|(pix, vec)| {
                    *vec = render_scene_pixel(&config, &scene, lights.as_ref(), pix)
                });

            if let Some(buffer) = show_frame_buffer(&config, frame_buffer.as_slice()) {
                window
                    .set_image(
                        "frame",
                        ImageView::new(
                            ImageInfo::rgb8(config.width as u32, config.height as u32),
                            buffer.as_ref(),
                        )
                        .borrow(),
                    )
                    .expect("Failed to show new frame");
            }
            frame_count += 1;
        }
    })
}
