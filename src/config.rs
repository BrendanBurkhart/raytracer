use clap::{crate_authors, crate_version, App, Arg};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path;

use super::linear;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputConfig {
    pub image_width: u32,
    pub image_height: u32,
    pub image_file: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CameraConfig {
    pub view_width: f64,

    pub position: linear::Vector,
    pub target: linear::Vector,

    #[serde(default)]
    pub roll: f64,

    pub focal_length: f64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub camera: CameraConfig,
    pub output: OutputConfig,

    pub maximum_reflections: u32,

    pub model_file: String,
    pub lighting_file: String,
}

fn parse_config_file(config_file: &path::Path) -> Result<Config, io::Error> {
    let f = fs::File::open(config_file)?;
    let config: Config = serde_json::from_reader(f)?;

    Ok(config)
}

pub fn configure() -> Result<Config, io::Error> {
    let matches = App::new("raytracer")
        .about("A simple ray tracer")
        .author(crate_authors!())
        .version(crate_version!())
        .arg(
            Arg::new("config")
                .about("Config file specifying models, lighting, etc.")
                .required(true)
                .index(1)
        )
        .arg(
            Arg::new("image_width")
                .about("Width of output image")
                .short('w')
                .takes_value(true)
        )
        .arg(
            Arg::new("image_height")
                .about("Height of output image")
                .short('h')
                .takes_value(true)

        )
        .get_matches();

    let mut config = parse_config_file(path::Path::new(matches.value_of("config").unwrap()))?;

    if let Some(ref image_width_str) = matches.value_of("image_width") {
        let image_width: u32 = image_width_str
            .parse()
            .expect("Output image width must be an unsigned integer");
        config.output.image_width = image_width;
    }

    if let Some(ref image_height_str) = matches.value_of("image_height") {
        let image_height: u32 = image_height_str
            .parse()
            .expect("Output image height must be an unsigned integer");
        config.output.image_height = image_height;
    }

    Ok(config)
}
