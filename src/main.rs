mod linear;
mod load;
mod render;
mod scene;

use std::env;
use std::path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please specify rendering task description file");
        return;
    }

    let rendering_task = load::load_scene(path::Path::new(&args[1])).expect("Scene file should load successfully");

    let image = rendering_task.execute();
    image
        .save_with_format(path::Path::new(&rendering_task.output_file), image::ImageFormat::Png)
        .expect("Saving output file should succeed!");
}
