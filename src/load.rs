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

fn parse_lights(lighting_file: &str) -> Result<Vec<scene::lighting::LightSource>, io::Error> {
    let f = fs::File::open(path::Path::new(lighting_file))?;
    let data: LightingData = serde_json::from_reader(f)?;

    Ok(data.lights)
}

pub fn scene(lighting_file: &str, model_file: &str) -> Result<scene::Scene, io::Error> {
    let lights = parse_lights(&lighting_file).expect("Lighting config should load correctly");

    let (materials, objects) = wavefront::load_obj(path::Path::new(&model_file))
        .expect("OBJ/MTL model files should load correctly");

    Ok(scene::Scene::new(materials, objects, lights))
}
