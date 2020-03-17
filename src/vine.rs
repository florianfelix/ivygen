use nalgebra::{Point3, Vector3};
use ncollide3d::shape::Capsule;
use oorandom::Rand32;

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
            shape: Capsule::new(0.2, 0.1),
        }
    }
    pub fn set_grow_direction(&mut self, last: &VineElement) {
        self.grow_direction = (self.position - last.position).normalize();
    }
    pub fn set_surface_direction(&mut self) {
        // IMPLEMENT ME
    }
}

#[derive(Debug)]
pub struct VineSettings {
    pub rng: Rand32,
    pub count: usize,
    pub part_length: f32,
    pub weight_random: f32,
    pub weight_up: f32,
    pub weight_direction: f32,
}

impl VineSettings {
    pub fn new(
        // rng: Rand32,
        count: usize,
        part_length: f32,
        weight_random: f32,
        weight_up: f32,
        weight_direction: f32,
    ) -> VineSettings {
        VineSettings {
            rng: Rand32::new_inc(1, 2),
            count,
            part_length,
            weight_random,
            weight_up,
            weight_direction,
        }
    }
}
