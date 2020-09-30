use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path;

use super::linear;
use super::render;
use super::scene;

mod wavefront;

#[derive(Serialize, Deserialize)]
struct CameraData {
    image_width: u32,
    image_height: u32,
    view_width: f64,
    view_height: f64,
    camera_position: linear::Vector,
    camera_target: linear::Vector,
    camera_roll: f64,
    focal_length: f64,
}

fn parse_camera(file: &path::Path) -> Result<render::Camera, io::Error> {
    let f = fs::File::open(file)?;
    let data: CameraData = serde_json::from_reader(f)?;

    let scope =
        render::lens::Scope::new(data.camera_target, data.camera_position, data.camera_roll);

    let lens = render::lens::PerspectiveLens::new(
        data.view_width,
        data.view_height,
        scope,
        data.focal_length,
    );

    Ok(render::Camera::new(
        data.image_width,
        data.image_height,
        Box::new(lens),
    ))
}

#[derive(Serialize, Deserialize)]
struct LightingData {
    lights: Vec<scene::lighting::LightSource>,
}

fn parse_lights(file: &path::Path) -> Result<Vec<scene::lighting::LightSource>, io::Error> {
    let f = fs::File::open(file)?;
    let data: LightingData = serde_json::from_reader(f)?;

    Ok(data.lights)
}

#[derive(Serialize, Deserialize)]
struct SceneDescription {
    model_file: String,
    camera_file: String,
    lighting_file: String,
    output_file: String,
}

fn parse_scene_description(file: &path::Path) -> Result<SceneDescription, io::Error> {
    let f = fs::File::open(file)?;
    let description: SceneDescription = serde_json::from_reader(f)?;

    Ok(description)
}

pub struct RenderTask {
    pub scene: scene::Scene,
    pub camera: render::Camera,
    pub output_file: String,
}

pub fn load_scene(scene_file: &path::Path) -> Result<RenderTask, io::Error> {
    let scene_description =
        parse_scene_description(scene_file).expect("Scene file should load correctly");

    let camera = parse_camera(path::Path::new(&scene_description.camera_file))
        .expect("Camera config should load correctly");
    let lights = parse_lights(path::Path::new(&scene_description.lighting_file))
        .expect("Lighting config should load correctly");

    let (materials, objects) = wavefront::load_obj(path::Path::new(&scene_description.model_file))
        .expect("OBJ/MTL model files should load correctly");

    let full_scene = scene::Scene::new(materials, objects, lights);

    let task = RenderTask {
        scene: full_scene,
        camera,
        output_file: scene_description.output_file,
    };

    Ok(task)
}
