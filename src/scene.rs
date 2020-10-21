use super::linear;
use std::f64;

pub mod lighting;
pub mod primitive;

pub struct Scene {
    pub materials: Vec<lighting::Material>,
    pub objects: Vec<primitive::Triangle>,
}

impl Scene {
    pub fn new(
        materials: Vec<lighting::Material>,
        objects: Vec<primitive::Triangle>,
    ) -> Scene {
        Scene {
            materials,
            objects,
        }
    }

    pub fn find_intersection<'a>(
        &'a self,
        ray: &linear::Ray,
    ) -> (Option<&'a primitive::Triangle>, f64, f64, f64) {
        let mut t = f64::MAX;
        let mut intersection = Option::None;
        let mut a = 0.0;
        let mut b = 0.0;

        self.objects.iter().for_each(|object| {
            let (intersected, temp_t, temp_a, temp_b) = object.intersect(ray, t);
            if intersected {
                intersection = Some(object);
                t = temp_t;
                a = temp_a;
                b = temp_b;
            }
        });

        (intersection, t, a, b)
    }
}
