use crate::linear_algebra as la;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct UV {
    pub u: f64,
    pub v: f64,
}

impl UV {
    pub fn new(u: f64, v: f64) -> UV {
        UV { u, v }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Color(f64, f64, f64);

impl Color {
    pub fn black() -> Color {
        Color(0.0, 0.0, 0.0)
    }

    pub fn new(red: f64, green: f64, blue: f64) -> Color {
        Color(red, green, blue)
    }

    fn combine(coef: f64, first: Color, second: Color) -> Color {
        Color(
            coef * first.0 * second.0,
            coef * first.1 * second.1,
            coef * first.2 * second.2,
        )
    }

    pub fn add(&mut self, other: Color) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }

    pub fn to_pixel(&self) -> (u8, u8, u8) {
        let red = (self.0 * 255.0).min(255.0) as u8;
        let green = (self.1 * 255.0).min(255.0) as u8;
        let blue = (self.2 * 255.0).min(255.0) as u8;

        (red, green, blue)
    }
}

pub struct Texture {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl Texture {
    pub fn new(data: Vec<u8>, width: usize, height: usize) -> Texture {
        Texture {
            data,
            width,
            height,
        }
    }

    fn color_at(&self, uv: UV) -> Color {
        let u = uv.u.rem_euclid(1.0);
        let v = uv.v.rem_euclid(1.0);
        let x = (u * (self.width as f64)) as usize;
        let y = ((1.0 - v) * (self.height as f64)) as usize;

        let index = 3 * (y * self.width + x);

        if index >= 3 * self.width * self.height {
            println!("{}, {}", u, v);
            return Color::black();
        }

        let r = self.data[index] as f64;
        let g = self.data[index + 1] as f64;
        let b = self.data[index + 2] as f64;

        Color(r / 255.0, g / 255.0, b / 255.0)
    }
}

pub struct Material {
    specular: Color,
    diffuse: Color,
    ambient: Color,
    alpha: f64,
    reflectance: f64,
    transparency: f64,
    index_of_refraction: f64,
    texture: Texture,
}

impl Material {
    pub fn new(
        specular: Color,
        diffuse: Color,
        ambient: Color,
        alpha: f64,
        reflectance: f64,
        transparency: f64,
        index_of_refraction: f64,
        texture: Texture,
    ) -> Material {
        Material {
            specular,
            diffuse,
            ambient,
            alpha,
            reflectance,
            transparency,
            index_of_refraction,
            texture,
        }
    }
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct LightSource {
    pub position: la::Vector,
    specular: Color,
    diffuse: Color,
    ambient: Color,
}

impl LightSource {
    pub fn calculate_ambient(lights: &Vec<LightSource>) -> Color {
        let mut red = 0.0;
        let mut green = 0.0;
        let mut blue = 0.0;

        for light in lights {
            red = red + light.ambient.0;
            green = green + light.ambient.1;
            blue = blue + light.ambient.2;
        }

        Color(red, green, blue)
    }
}

pub fn calculate(
    lights: &Vec<LightSource>,
    ambient_light: Color,
    ray: &la::Ray,
    normal: la::Vector,
    light_strength: f64,
    material: &Material,
) -> (Color, f64, Vec<la::Ray>) {
    let mut color = Color(0.0, 0.0, 0.0);

    for light in lights {
        let dist = light.position.subtract(&ray.position);

        if normal.dot(&dist) <= 0.0 {
            continue;
        }

        let dist = dist.normalize();

        let reflection = dist.reflect_across(&normal);

        let diffuse_coef = dist.dot(&normal);
        let diffuse = Color::combine(
            diffuse_coef * light_strength,
            light.diffuse,
            material.diffuse,
        );
        color.add(diffuse);

        let mut specular_base = reflection.dot(&ray.direction.negative());
        if specular_base < 0.0 {
            specular_base = 0.0;
        }

        let specular_coef = specular_base.powf(material.alpha);

        let specular = Color::combine(
            specular_coef * light_strength,
            light.specular,
            material.specular,
        );
        color.add(specular);
    }

    color.add(Color::combine(
        light_strength,
        ambient_light,
        material.ambient,
    ));

    let reflection = la::Ray {
        position: ray.position,
        direction: ray.direction.negative().normalize().reflect_across(&normal),
    };

    (
        color,
        light_strength * material.reflectance,
        vec![reflection],
    )
}

pub fn calculate_with_tex(
    lights: &Vec<LightSource>,
    ambient_light: Color,
    ray: &la::Ray,
    uv: UV,
    normal: la::Vector,
    light_strength: f64,
    material: &Material,
) -> (Color, f64, Vec<la::Ray>) {
    let mut color = Color(0.0, 0.0, 0.0);

    for light in lights {
        let dist = light.position.subtract(&ray.position);

        if normal.dot(&dist) <= 0.0 {
            continue;
        }

        let dist = dist.normalize();

        let reflection = dist.reflect_across(&normal);

        let diffuse_coef = dist.dot(&normal);
        let diffuse = Color::combine(1.0, material.texture.color_at(uv), material.diffuse);
        let diffuse = Color::combine(diffuse_coef * light_strength, light.diffuse, diffuse);
        color.add(diffuse);

        let mut specular_base = reflection.dot(&ray.direction.negative());
        if specular_base < 0.0 {
            specular_base = 0.0;
        }

        let specular_coef = specular_base.powf(material.alpha);

        let specular = Color::combine(
            specular_coef * light_strength,
            light.specular,
            material.specular,
        );
        color.add(specular);
    }

    color.add(Color::combine(
        light_strength,
        ambient_light,
        material.ambient,
    ));

    let reflection = la::Ray {
        position: ray.position,
        direction: ray.direction.negative().normalize().reflect_across(&normal),
    };

    (
        color,
        light_strength * material.reflectance,
        vec![reflection],
    )
}
