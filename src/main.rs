extern crate opencv;

mod linear;
mod config;
mod load;
mod render;
mod scene;

use std::{
    f64::consts,
    io::Write,
};

use opencv::{
    core,
    videoio,
    prelude::*,
};

fn write_frame(input: &image::RgbImage, output: &mut videoio::VideoWriter) {
    let width = input.width() as i32;
    let height = input.height() as i32;

    let mut frame = core::Mat::new_rows_cols_with_default(height, width, core::Vec3b::typ(), core::Scalar::all(0.0)).expect("Should be able to create mat");

    for i in 0..height {
        for j in 0..width {
            let p = input.get_pixel(j as u32, i as u32);
            let pixel = frame.at_mut::<core::Vec3b>(i * width + j).unwrap();
            *pixel = core::Vec3b::from([p[0], p[1], p[2]]);
        }
    }

    output.write(&frame).unwrap();
}

fn render_frame(render_config: &config::Config, scene: &scene::Scene, lighting: &scene::lighting::Lighting, output: &mut videoio::VideoWriter, theta: f64) {
    const R: f64 =  32.0;
    const Y: f64 = 10.0;

    let position = linear::Vector::new(R*theta.cos(), Y, R*theta.sin());

    let camera_scope = render::lens::Scope::new(render_config.camera.target, position, render_config.camera.roll);

    let lens = render::lens::PerspectiveLens::new(
        render_config.camera.view_width,
        render_config.output.image_width,
        render_config.output.image_height,
        camera_scope,
        render_config.camera.focal_length,
    );

    let lighting = lighting.rotate(theta);

    let task = render::RenderTask {
        scene,
        lighting: &lighting,
        lens:  &lens,
        image_width: render_config.output.image_width,
        image_height: render_config.output.image_height,
        max_reflections: render_config.maximum_reflections,
    };

    write_frame(&task.execute(), output);
}

fn main() {
    let render_config = config::configure().expect("Configuration must succeed");

    let scene_data = load::scene(&render_config.model_file).expect("Scene data must load");
    let lighting = load::lighting(&render_config.lighting_file).expect("Scene lighting must load");

    let fourcc = videoio::VideoWriter::fourcc('M' as i8, 'J' as i8, 'P' as i8, 'G' as i8).unwrap();
    let mut output = videoio::VideoWriter::new("output.avi", fourcc, 30.0, core::Size::new(render_config.output.image_width as i32, render_config.output.image_height as i32), true).expect("Should be able to create video output");
    let steps = 300;

    print!("\rProgress: 0%");
    std::io::stdout().flush().unwrap();

    for i in 0..steps {
        let percent = (i as f64)/(steps as f64);
        let theta = percent * 2.0 * consts::PI;

        render_frame(&render_config, &scene_data, &lighting, &mut output, theta);

        print!("\rProgress: {}%", (percent * 100.0) as i32);
        std::io::stdout().flush().unwrap();
    }

    print!("\rProgress: 100%");
    println!("\nFinished");
}
