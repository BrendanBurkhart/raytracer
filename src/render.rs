use image;

use super::linear;
use super::scene;

pub mod lens;
pub mod camera;

pub struct RenderTask<'a> {
    pub scene: &'a scene::Scene,
    pub lighting: &'a scene::lighting::Lighting,
    pub lens: &'a dyn lens::Lens,
    pub image_width: u32,
    pub image_height: u32,
    pub max_reflections: u32,
}

impl RenderTask<'_> {
    pub fn execute(&self) -> image::RgbImage {
        let camera = camera::Camera::new(self.image_width, self.image_height, self.lens);

        camera.render(self.scene, self.lighting, self.max_reflections)
    }
}
