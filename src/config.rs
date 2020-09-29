use crate::camera;
use crate::lens;
use crate::lighting;
use crate::linear_algebra as la;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path;

#[derive(Serialize, Deserialize)]
struct CameraData {
    image_width: u32,
    image_height: u32,
    view_width: f64,
    view_height: f64,
    camera_position: la::Vector,
    camera_target: la::Vector,
    camera_roll: f64,
    focal_length: f64,
}

pub fn parse_camera(file: &path::Path) -> Result<camera::Camera, io::Error> {
    let f = fs::File::open(file)?;
    let data: CameraData = serde_json::from_reader(f)?;

    let scope = lens::Scope::new(data.camera_target, data.camera_position, data.camera_roll);

    let lens =
        lens::PerspectiveLens::new(data.view_width, data.view_height, scope, data.focal_length);

    Ok(camera::Camera::new(
        data.image_width,
        data.image_height,
        Box::new(lens),
    ))
}

#[derive(Serialize, Deserialize)]
struct LightingData {
    lights: Vec<lighting::LightSource>,
}

pub fn parse_lights(file: &path::Path) -> Result<Vec<lighting::LightSource>, io::Error> {
    let f = fs::File::open(file)?;
    let data: LightingData = serde_json::from_reader(f)?;

    Ok(data.lights)
}
