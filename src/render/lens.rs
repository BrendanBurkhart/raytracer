use super::linear;

struct ViewPort {
    width: f64,
    height: f64,
}

#[derive(Debug)]
pub struct Scope {
    position: linear::Vector,
    right: linear::Vector,
    up: linear::Vector,
    forward: linear::Vector,
}

impl Scope {
    pub fn new(target: linear::Vector, position: linear::Vector, roll: f64) -> Scope {
        let forward = target.subtract(&position).normalize();
        let vertical = linear::Vector::new(0.0, 1.0, 0.0);

        let right: linear::Vector;
        if forward.equals(&vertical) {
            right = linear::Vector::new(1.0, 0.0, 0.0);
        } else {
            right = forward.cross(&vertical);
        }

        let up = right.cross(&forward);
        let up = up.rotate(-roll, &forward);

        let right = forward.cross(&up);

        Scope {
            position,
            right,
            up,
            forward,
        }
    }
}

pub trait Lens: Sync + Send {
    fn generate_light_ray(&self, x: f64, y: f64) -> linear::Ray;
}

pub struct OrthographicLens {
    view_port: ViewPort,
    scope: Scope,
}

impl OrthographicLens {
    pub fn new(width: f64, image_width: f64, image_height: f64, scope: Scope) -> OrthographicLens {
        let view_port = ViewPort {
            width,
            height: width * (image_height / image_width),
        };

        OrthographicLens { view_port, scope }
    }
}

impl Lens for OrthographicLens {
    fn generate_light_ray(&self, x: f64, y: f64) -> linear::Ray {
        let horizontal = self.scope.right.scale(x * self.view_port.width * 0.5);
        let vertical = self.scope.up.scale(y * self.view_port.height * 0.5);

        linear::Ray {
            position: self.scope.position.add(&horizontal).add(&vertical),
            direction: self.scope.forward,
        }
    }
}

pub struct PerspectiveLens {
    view_port: ViewPort,
    scope: Scope,
    focal_length: f64,
}

impl PerspectiveLens {
    pub fn new(
        width: f64,
        image_width: u32,
        image_height: u32,
        scope: Scope,
        focal_length: f64,
    ) -> PerspectiveLens {
        let view_port = ViewPort {
            width,
            height: width * ((image_height as f64) / (image_width as f64)),
        };

        PerspectiveLens {
            view_port,
            scope,
            focal_length,
        }
    }
}

impl Lens for PerspectiveLens {
    fn generate_light_ray(&self, x: f64, y: f64) -> linear::Ray {
        let forward = self.scope.forward.scale(self.focal_length);
        let horizontal = self.scope.right.scale(x * self.view_port.width * 0.5);
        let vertical = self.scope.up.scale(y * self.view_port.height * 0.5);

        let direction = forward.add(&horizontal).add(&vertical).normalize();

        linear::Ray {
            position: self.scope.position,
            direction,
        }
    }
}
