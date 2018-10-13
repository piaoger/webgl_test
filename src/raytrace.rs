/*
 * Example using *RsCSG* with *smallpt* (raytracer).
 *
 * Warning, the output of this example is not good looking =/
 */

// extern crate png;
// extern crate rscsg;
// extern crate smallpt;

use rscsg::dim3::{Csg, Vector};

use png::HasParameters;
use smallpt::{
    saturate,
    tonemap,
    trace,
    Camera,
    Material,
    Rectangle,
    Scene,
    Triangle,
    Vec3,
    BSDF,
};
use std::{fs::File, io::BufWriter, path::Path};

// mesh
use js;
use png;
use std::io::Read;

const WIDTH: usize = 24;
const HEIGHT: usize = WIDTH;

pub fn render() -> Vec<u8> {
    println!("render scene...");

    let mut backbuffer = vec![Vec3::zeros(); WIDTH * HEIGHT];
    let scene = create_scene();

    let aperture = 0.5135;
    let camera_origin = Vec3::new(50.0, 50.0, 300.0);
    let camera_direction = Vec3::new(0.0, -0.05, -1.0).normalize();
    let camera_right =
        Vec3::new(WIDTH as f32 * aperture / HEIGHT as f32, 0.0, 0.0);
    let camera_up =
        camera_right.cross(&camera_direction).normalize() * aperture;

    let camera =
        Camera::new(camera_origin, camera_direction, camera_right, camera_up);

    let mut num_rays = 0;
    js::log("Ray tracing...");

    let callback = |msg: &str| {
        js::log(&msg);
    };

    trace(
        &scene,
        &camera,
        WIDTH,
        HEIGHT,
        512,
        &mut backbuffer,
        &mut num_rays,
        callback,
    );

    js::log("Creating bitmap...");
    let bitmap: Vec<u8> = backbuffer
        .iter()
        .flat_map(|&comp| {
            let adjust = saturate(tonemap(comp));
            vec![
                (adjust.x * 255.0).round() as u8,
                (adjust.y * 255.0).round() as u8,
                (adjust.z * 255.0).round() as u8,
                0xffu8,
            ]
        })
        .collect();

    let mut png_data: Vec<u8> = vec![];

    {
        let mut encoder =
            png::Encoder::new(&mut png_data, WIDTH as u32, HEIGHT as u32);
        encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();

        writer.write_image_data(&bitmap).unwrap();

        js::log("redering Done");
    }

    png_data.to_vec()
}

fn create_scene() -> Scene {
    js::log("ceate scene....");
    let cube = Csg::cube(Vector(80., 80., 80.), true);
    let sphere = Csg::sphere(80., 10, 10);

    let cuber = Csg::subtract(&cube, &sphere);

    let mut scene = Scene::init();

    let triangle_material =
        Material::new(Vec3::zeros(), Vec3::new(0.4, 0.4, 1.0), BSDF::Diffuse);

    cuber.iter_triangles(|tri| {
        scene.add(Box::new(Triangle::new(
            vec_to_vec3(tri.positions[0]),
            vec_to_vec3(tri.positions[1]),
            vec_to_vec3(tri.positions[2]),
            triangle_material,
        )));
    });

    // Light
    scene.add(Box::new(Rectangle::new(
        Vec3::new(50.0, 81.5, 50.0),
        Vec3::new(0.0, -1.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        133.0,
        133.0,
        Material::new(
            Vec3::new(12.0, 12.0, 12.0),
            Vec3::zeros(),
            BSDF::Diffuse,
        ),
    )));

    js::log("create scene Done");

    // disable bvh for call stack error
    // RangeError: Maximum call stack size exceeded
    let enable_bvh = false;
    if enable_bvh {
        scene.build_bvh();
    }

    return scene;
}

fn vec_to_vec3(v: Vector) -> Vec3 {
    Vec3::new(v.0, v.1, v.2)
}
