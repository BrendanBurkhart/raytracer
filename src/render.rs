use image;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

use super::linear;
use super::scene;

pub mod lens;

pub struct RenderTask {
    pub scene: scene::Scene,
    pub camera: Camera,
    pub output_file: String,
}

impl RenderTask {
    pub fn execute(&self) -> image::RgbImage {
        self.camera.render(&self.scene, 15)
    }
}

pub struct Camera {
    image_width: u32,
    image_height: u32,
    size: usize,
    depth: usize,
    lens: Box<dyn lens::Lens + Sync + Send + 'static>,
}

impl Camera {
    pub fn new(
        image_width: u32,
        image_height: u32,
        lens: Box<dyn lens::Lens + Sync + Send + 'static>,
    ) -> Camera {
        let size = (image_width * image_height) as usize;

        Camera {
            image_width,
            image_height,
            size,
            depth: 3,
            lens,
        }
    }

    fn render(&self, scene: &scene::Scene, max_reflections: u32) -> image::RgbImage {
        let mut output = std::iter::repeat(0 as u8)
            .take(self.depth * self.size)
            .collect::<Vec<_>>();

        let mut rows = Vec::new();

        let mut remaining = &mut output[..];

        let block_size = (self.image_width as usize) * self.depth;

        for y in 0..self.image_height {
            let (row, new_remaining) = remaining.split_at_mut(block_size);
            rows.push((y, row));
            remaining = new_remaining;
        }

        let progress = Arc::new(Mutex::new(0));

        rows.par_iter_mut().for_each(|(pixel_y, row)| {
            let screen_y = -2.0 * (*pixel_y as f64) / (self.image_height as f64) + 1.0;

            for pixel_x in 0..self.image_width {
                let screen_x = 2.0 * (pixel_x as f64) / (self.image_width as f64) - 1.0;
                let ray = self.lens.generate_light_ray(screen_x, screen_y);

                let color = self.trace_ray(scene, ray, 1.0, max_reflections).to_pixel();

                let index = self.depth * (pixel_x as usize);
                row[index] = color.0;
                row[index + 1] = color.1;
                row[index + 2] = color.2;
            }

            let mut progress = progress.lock().unwrap();
            *progress += 1;

            let percent = (*progress as f64) / (self.image_height as f64) * 100.0;
            print!("\rProgress: {}", percent as i32);
        });

        println!("\nFinished");

        image::ImageBuffer::from_raw(self.image_width, self.image_height, output)
            .expect("Should create image successfully")
    }

    fn trace_ray(
        &self,
        scene: &scene::Scene,
        ray: linear::Ray,
        light_strength: f64,
        remaining_reflections: u32,
    ) -> scene::lighting::Color {
        let (intersection, t, b, c) = scene.find_intersection(&ray);
        let object = match intersection {
            Some(object) => object,
            None => return scene::lighting::Color::black(),
        };

        let distance = ray.direction.scale(t);
        let intersection_point = ray.position.add(&distance);
        let ray = linear::Ray {
            position: intersection_point,
            direction: ray.direction,
        };
        let normal = object.surface_normal(b, c);
        let material = &scene.materials[object.material_id()];

        let visible_lights = self.find_visible_lights(scene, intersection_point);

        let (mut surface_color, light_strength, rays) = match object.has_texture() {
            false => scene::lighting::calculate(
                &visible_lights,
                scene.ambient_light,
                &ray,
                normal,
                light_strength,
                material,
            ),
            true => scene::lighting::calculate_with_tex(
                &visible_lights,
                scene.ambient_light,
                &ray,
                object.uv(b, c),
                normal,
                light_strength,
                material,
            ),
        };

        if remaining_reflections > 0 {
            for ray in rays {
                let reflected_color =
                    self.trace_ray(scene, ray, light_strength, remaining_reflections - 1);

                surface_color.add(reflected_color);
            }
        }

        surface_color
    }

    fn find_visible_lights(
        &self,
        scene: &scene::Scene,
        position: linear::Vector,
    ) -> Vec<scene::lighting::LightSource> {
        let mut visible_lights: Vec<scene::lighting::LightSource> = Vec::new();
        for light in &scene.lights {
            let light_ray = linear::Ray {
                position,
                direction: light.position.subtract(&position),
            };

            let (_, distance, _, _) = scene.find_intersection(&light_ray);
            if distance >= 1.0 {
                visible_lights.push(*light);
            }
        }

        visible_lights
    }
}
