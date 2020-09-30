use approx;
use serde::{Deserialize, Serialize};
use std::f64::consts;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z }
    }

    pub fn dot(&self, other: &Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vector) -> Vector {
        Vector::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn magnitude(&self) -> f64 {
        (self.dot(self)).sqrt()
    }

    pub fn add(&self, other: &Vector) -> Vector {
        Vector::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    pub fn subtract(&self, other: &Vector) -> Vector {
        Vector::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    pub fn negative(&self) -> Vector {
        Vector::new(-self.x, -self.y, -self.z)
    }

    pub fn scale(&self, scale: f64) -> Vector {
        Vector::new(scale * self.x, scale * self.y, scale * self.z)
    }

    pub fn normalize(&self) -> Vector {
        let magnitude = self.magnitude();
        if magnitude == 0.0 {
            panic!("Can't normalize zero-length vector");
        }

        self.scale(1.0 / magnitude)
    }

    pub fn rotate(&self, degrees: f64, axis: &Vector) -> Vector {
        let axis = axis.normalize();

        let angle = degrees * consts::PI / 180.0;

        let matrix = [
            angle.cos() + axis.x * axis.x * (1.0 - angle.cos()),
            axis.x * axis.y * (1.0 - angle.cos()) - axis.z * angle.sin(),
            axis.x * axis.z * (1.0 - angle.cos()) + axis.y * angle.sin(),
            axis.y * axis.x * (1.0 - angle.cos()) + axis.z * angle.sin(),
            angle.cos() + axis.y * axis.y * (1.0 - angle.cos()),
            axis.y * axis.z * (1.0 - angle.cos()) - axis.x * angle.sin(),
            axis.z * axis.x * (1.0 - angle.cos()) - axis.y * angle.sin(),
            axis.z * axis.y * (1.0 - angle.cos()) + axis.x * angle.sin(),
            angle.cos() + axis.z * axis.z * (1.0 - angle.cos()),
        ];

        Vector::new(
            matrix[3 * 0 + 0] * self.x + matrix[3 * 0 + 1] * self.y + matrix[3 * 0 + 2] * self.z,
            matrix[3 * 1 + 0] * self.x + matrix[3 * 1 + 1] * self.y + matrix[3 * 1 + 2] * self.z,
            matrix[3 * 2 + 0] * self.x + matrix[3 * 2 + 1] * self.y + matrix[3 * 2 + 2] * self.z,
        )
    }

    pub fn reflect_across(&self, axis: &Vector) -> Vector {
        let delta = axis.scale(2.0 * self.dot(axis));

        delta.subtract(self)
    }

    pub fn equals(&self, other: &Vector) -> bool {
        approx::abs_diff_eq!(self.x, other.x)
            && approx::abs_diff_eq!(self.y, other.y)
            && approx::abs_diff_eq!(self.z, other.z)
    }
}

pub struct Ray {
    pub position: Vector,
    pub direction: Vector,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dot() {
        let v1 = Vector {
            x: 1.0,
            y: 0.0,
            z: 1.0,
        };
        let v2 = Vector {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        let d = v1.dot(&v2);
        approx::assert_ulps_eq!(0.0, d);

        let v2 = Vector {
            x: 2.0,
            y: 0.0,
            z: 0.0,
        };
        let d = v1.dot(&v2);
        approx::assert_ulps_eq!(2.0, d);
    }

    #[test]
    fn cross() {
        let v1 = Vector {
            x: 1.0,
            y: 0.0,
            z: 1.0,
        };
        let v2 = Vector {
            x: -1.0,
            y: 0.0,
            z: 1.0,
        };
        let v3 = v1.cross(&v2);
        approx::assert_ulps_eq!(0.0, v3.x);
        approx::assert_ulps_eq!(-2.0, v3.y);
        approx::assert_ulps_eq!(0.0, v3.z);

        let v2 = Vector {
            x: 0.0,
            y: 3.0,
            z: -5.0,
        };
        let v3 = v1.cross(&v2);
        approx::assert_ulps_eq!(-3.0, v3.x);
        approx::assert_ulps_eq!(5.0, v3.y);
        approx::assert_ulps_eq!(3.0, v3.z);
    }

    #[test]
    fn magnitude() {
        let v = Vector {
            x: 0.0,
            y: 0.0,
            z: -9.0,
        };
        approx::assert_ulps_eq!(9.0, v.magnitude());

        let v = Vector {
            x: -6.0,
            y: 5.0,
            z: -3.0,
        };
        approx::assert_ulps_eq!((70.0 as f64).sqrt(), v.magnitude());
    }

    #[test]
    fn add() {
        let v1 = Vector {
            x: -5.0,
            y: 3.0,
            z: -9.0,
        };
        let v2 = Vector {
            x: -1.0,
            y: 0.0,
            z: 6.0,
        };

        let v3 = v1.add(&v2);
        approx::assert_ulps_eq!(-6.0, v3.x);
        approx::assert_ulps_eq!(3.0, v3.y);
        approx::assert_ulps_eq!(-3.0, v3.z);
    }

    #[test]
    fn subtract() {
        let v1 = Vector {
            x: -5.0,
            y: 3.0,
            z: -9.0,
        };
        let v2 = Vector {
            x: -1.0,
            y: 0.0,
            z: 6.0,
        };

        let v3 = v1.subtract(&v2);
        approx::assert_ulps_eq!(-4.0, v3.x);
        approx::assert_ulps_eq!(3.0, v3.y);
        approx::assert_ulps_eq!(-15.0, v3.z);
    }

    #[test]
    fn negative() {
        let v1 = Vector {
            x: -5.0,
            y: 3.0,
            z: -9.0,
        };
        let v2 = v1.negative();

        approx::assert_ulps_eq!(5.0, v2.x);
        approx::assert_ulps_eq!(-3.0, v2.y);
        approx::assert_ulps_eq!(9.0, v2.z);
    }

    #[test]
    fn scale() {
        let v1 = Vector {
            x: -5.0,
            y: 3.0,
            z: -9.0,
        };
        let v2 = v1.scale(-2.0);

        approx::assert_ulps_eq!(10.0, v2.x);
        approx::assert_ulps_eq!(-6.0, v2.y);
        approx::assert_ulps_eq!(18.0, v2.z);
    }

    #[test]
    fn normalize() {
        let v1 = Vector {
            x: 0.0,
            y: -1.0,
            z: 0.0,
        };

        let v2 = v1.normalize();
        approx::assert_ulps_eq!(0.0, v2.x);
        approx::assert_ulps_eq!(-1.0, v2.y);
        approx::assert_ulps_eq!(0.0, v2.z);

        let v1 = Vector {
            x: 5.0,
            y: -5.0,
            z: 5.0,
        };

        let v2 = v1.normalize();
        approx::assert_ulps_eq!(1.0 / (3.0 as f64).sqrt(), v2.x);
        approx::assert_ulps_eq!(-1.0 / (3.0 as f64).sqrt(), v2.y);
        approx::assert_ulps_eq!(1.0 / (3.0 as f64).sqrt(), v2.z);
    }

    #[test]
    fn rotate() {
        let v1 = Vector {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };

        let axis = Vector {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };

        let v2 = v1.rotate(90.0, &axis);
        approx::assert_ulps_eq!(0.0, v2.x);
        approx::assert_ulps_eq!(0.0, v2.y);
        approx::assert_ulps_eq!(-1.0, v2.z);

        let v1 = Vector {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };

        let axis = Vector {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };

        let v2 = v1.rotate(120.0, &axis);
        approx::assert_ulps_eq!(0.0, v2.x);
        approx::assert_ulps_eq!(1.0, v2.y);
        approx::assert_ulps_eq!(0.0, v2.z);

        let v1 = Vector {
            x: -34.0,
            y: 13.0,
            z: 124.0,
        };

        let axis = Vector {
            x: -23.0,
            y: 345.0,
            z: 24.0,
        };

        let v2 = v1.rotate(0.0, &axis);
        approx::assert_ulps_eq!(v1.x, v2.x);
        approx::assert_ulps_eq!(v1.y, v2.y);
        approx::assert_ulps_eq!(v1.z, v2.z);
    }

    #[test]
    fn reflect_across() {
        let axis = Vector {
            x: 0.0,
            y: -1.0,
            z: 0.0,
        };

        let v = Vector {
            x: -2.0,
            y: 3.0,
            z: -5.0,
        };

        let r = v.reflect_across(&axis);

        approx::assert_ulps_eq!(2.0, r.x);
        approx::assert_ulps_eq!(3.0, r.y);
        approx::assert_ulps_eq!(5.0, r.z);
    }
}
