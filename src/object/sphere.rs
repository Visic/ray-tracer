use nalgebra::{Point3, Unit, Vector3};

pub struct Sphere {
    pub location: Point3<f64>,
    pub radius: f64,
    pub color: [f32; 3],
}

impl Sphere {    
    pub fn hit_test(&self, ray: Unit<Vector3<f64>>) -> bool {
        let c = self.location.coords.dot(&self.location.coords)  - self.radius.powi(2);
        let v = ray.dot(&self.location.coords).powi(2) - c;
        v >= 0.0
    }
}