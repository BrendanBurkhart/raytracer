use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path;

use super::linear;
use super::scene;

mod wavefront;

#[derive(Serialize, Deserialize)]
struct LightingData {
    lights: Vec<scene::lighting::LightSource>,
}

pub fn lighting(lighting_file: &str) -> Result<scene::lighting::Lighting, io::Error> {
    let f = fs::File::open(path::Path::new(lighting_file)).expect("Lighting config should load correctly");
    let data: LightingData = serde_json::from_reader(f).expect("Lighting config should load correctly");

    Ok(scene::lighting::Lighting::new(data.lights))
}

pub fn scene(model_file: &str) -> Result<scene::Scene, io::Error> {
    let (materials, objects) = wavefront::load_obj(path::Path::new(&model_file))
        .expect("OBJ/MTL model files should load correctly");

    Ok(scene::Scene::new(materials, objects))
}
