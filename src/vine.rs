
use oorandom::Rand32;
use nalgebra::{Point3, Vector3};
use ncollide3d::shape::{Capsule, TriMesh};

use crate::context::Context;

#[derive(Debug, Clone)]
pub struct Vine <'a> {
    pub context: &'a Context,
    pub rng: Rand32,
    pub nodes: Vec<VineNode>,
    pub origin: Point3<f32>,
    depth: usize,
    max_children: usize,
}

impl<'a> Vine<'a> {
    pub fn new(context: &'a Context, depth: usize, origin: &Point3<f32>) -> Vine<'a> {
        Vine {
            context,
            depth,
            origin: origin.clone(),
            nodes: vec![],
            max_children: 3,
            rng: Rand32::new(0 + context.seed_base)
        }
    }
}



// VINENODE
#[derive(Debug, Clone)]
pub struct VineNode {
    pub position: Point3<f32>,
    pub shape: Capsule<f32>,
    pub vine_length: f32,
    pub floating_length: Option<f32>,
    pub sticking_since: Option<usize>,
    pub surface_direction: Option<Vector3<f32>>,
    pub grow_direction: Vector3<f32>,
}

impl VineNode {
    pub fn new() -> VineNode {
        VineNode {
            position: Point3::new(0.0, 0.0, 0.0),
            shape: Capsule::new(0.2, 0.1),
            vine_length: 0.0,
            floating_length: None,
            sticking_since: None,
            surface_direction: None,
            grow_direction: Vector3::new(0.0, 1.0, 0.0),
        }
    }
    pub fn set_grow_direction(&mut self, last: &VineNode) {
        self.grow_direction = (self.position - last.position).normalize();
    }
    pub fn set_surface_direction(&mut self) {
        // IMPLEMENT ME
    }
}