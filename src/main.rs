mod linear;
mod config;
mod load;
mod render;
mod scene;

use std::path;

fn main() {
    let render_config = config::configure().expect("Configuration must succeed");

    let scene_data = load::scene(&render_config.lighting_file, &render_config.model_file).expect("Scene data must load");

    let camera_scope = render::lens::Scope::new(render_config.camera.target, render_config.camera.position, render_config.camera.roll);

    let lens = render::lens::PerspectiveLens::new(
        render_config.camera.view_width,
        render_config.output.image_width,
        render_config.output.image_height,
        camera_scope,
        render_config.camera.focal_length,
    );

    let task = render::RenderTask {
        scene: &scene_data,
        lens:  &lens,
        image_width: render_config.output.image_width,
        image_height: render_config.output.image_height,
        max_reflections: render_config.maximum_reflections,
    };

    let image = task.execute();
    image
        .save_with_format(path::Path::new(&render_config.output.image_file), image::ImageFormat::Png)
        .expect("Saving output file should succeed!");
}
