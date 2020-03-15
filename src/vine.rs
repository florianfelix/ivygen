use nalgebra::{Point3, Vector3};
use ncollide3d::shape::Capsule;

#[derive(Debug)]
pub struct VineElement {
    position: Point3<f32>,
    grow_direction: Vector3<f32>,
    surface_direction: Option<Vector3<f32>>,
    shape: Capsule<f32>,
}

impl VineElement {
    pub fn new() -> VineElement {
        VineElement {
            position: Point3::new(0.0, 0.0, 0.0),
            grow_direction: Vector3::new(0.0, 1.0, 0.0),
            surface_direction: None,
            shape: Capsule::new(0.2, 0.1)
        }
    }
    pub fn set_grow_direction(&mut self, last: &VineElement) {
        self.grow_direction = (self.position - last.position).normalize();
    }
    pub fn set_surface_direction(&mut self) {
        // IMPLEMENT ME
    }
}