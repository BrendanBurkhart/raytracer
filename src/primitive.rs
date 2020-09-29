use crate::lighting;
use crate::linear_algebra as la;

pub struct Triangle {
    material_id: usize,
    normal: la::Vector,
    edge1: la::Vector,
    edge2: la::Vector,
    a: la::Vector,
    texture_map: Option<(lighting::UV, lighting::UV, lighting::UV)>,
    normal_map: Option<(la::Vector, la::Vector, la::Vector)>,
}

impl Triangle {
    pub fn new(
        a: la::Vector,
        b: la::Vector,
        c: la::Vector,
        material_id: usize,
        texture_map: Option<(lighting::UV, lighting::UV, lighting::UV)>,
        normal_map: Option<(la::Vector, la::Vector, la::Vector)>,
    ) -> Triangle {
        let edge1 = b.subtract(&a);
        let edge2 = c.subtract(&a);

        let normal = edge1.cross(&edge2).normalize();

        Triangle {
            material_id,
            normal,
            edge1,
            edge2,
            a,
            texture_map,
            normal_map,
        }
    }

    pub fn intersect(&self, ray: &la::Ray, max_range: f64) -> (bool, f64, f64, f64) {
        if self.normal.dot(&ray.direction) >= 0.0 {
            return (false, max_range, 0.0, 0.0);
        }

        let h = ray.direction.cross(&self.edge2);

        let det = self.edge1.dot(&h);
        if det < 1e-12 && det > -1e-12 {
            return (false, max_range, 0.0, 0.0);
        }

        let inv_det = 1.0 / det;

        let delta = ray.position.subtract(&self.a);

        let u = delta.dot(&h) * inv_det;
        if u < 0.0 || u > 1.0 {
            return (false, max_range, 0.0, 0.0);
        }

        let q = delta.cross(&self.edge1);

        let v = ray.direction.dot(&q) * inv_det;
        if v < 0.0 || (u + v) > 1.0 {
            return (false, max_range, 0.0, 0.0);
        }

        let t = self.edge2.dot(&q) * inv_det;
        if t > 1e-4 && t < max_range {
            return (true, t, u, v);
        }
        return (false, max_range, 0.0, 0.0);
    }

    pub fn surface_normal(&self, b: f64, c: f64) -> la::Vector {
        if !self.has_normal_map() {
            return self.normal;
        }

        let a = 1.0 - b - c;

        let normal_map = self.normal_map.unwrap();

        let a_vec = normal_map.0.scale(a);
        let b_vec = normal_map.1.scale(b);
        let c_vec = normal_map.2.scale(c);

        a_vec.add(&b_vec).add(&c_vec)
    }

    pub fn material_id(&self) -> usize {
        return self.material_id;
    }

    pub fn uv(&self, b: f64, c: f64) -> lighting::UV {
        let a = 1.0 - b - c;

        let texture_uv = self.texture_map.unwrap();

        let u = (a * texture_uv.0.u) + (b * texture_uv.1.u) + (c * texture_uv.2.u);
        let v = (a * texture_uv.0.v) + (b * texture_uv.1.v) + (c * texture_uv.2.v);

        lighting::UV::new(u, v)
    }

    pub fn has_texture(&self) -> bool {
        match self.texture_map {
            None => false,
            Some(_) => true,
        }
    }

    fn has_normal_map(&self) -> bool {
        match self.normal_map {
            None => false,
            Some(_) => true,
        }
    }
}
