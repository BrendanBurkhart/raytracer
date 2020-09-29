mod camera;
mod config;
mod lens;
mod lighting;
mod linear_algebra;
mod primitive;
mod scene;
mod stl;
mod wavefront;

use std::env;
use std::path;

fn main() {
    let args: Vec<String> = env::args().collect();

    let camera = config::parse_camera(path::Path::new("./camera.json"))
        .expect("Camera config should parse correctly");

    let lights = config::parse_lights(path::Path::new("./lighting.json"))
        .expect("Lighting config should parse correctly");

    let (materials, objects) =
        wavefront::load_obj(path::Path::new(&args[1])).expect("OBJ should load correctly");

    let ambient_light = lighting::LightSource::calculate_ambient(&lights);

    let scene = scene::Scene {
        materials,
        objects,
        lights,
        ambient_light,
    };

    let image = camera.render(&scene, 15);
    image
        .save_with_format(path::Path::new("./output.png"), image::ImageFormat::Png)
        .expect("Saving output file should succeed!");
}
