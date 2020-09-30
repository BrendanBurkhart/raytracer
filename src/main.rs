mod linear;
mod load;
mod render;
mod scene;

use std::env;
use std::path;

fn main() {
    let args: Vec<String> = env::args().collect();

    let task = load::load_scene(path::Path::new(&args[1])).expect("Scene file should load successfully");

    let image = task.camera.render(&task.scene, 15);
    image
        .save_with_format(path::Path::new(&task.output_file), image::ImageFormat::Png)
        .expect("Saving output file should succeed!");
}
