use nalgebra::{Point3, Vector3};
use ncollide3d::shape::{Capsule, TriMesh};
use oorandom::Rand32;

use crate::random_walk::grow_vector;

// IVY
// #[derive(Copy)]
pub struct Ivy {
    // pub settings: IvySettings,
    pub root: Vine,
    pub colliders: Vec<TriMesh<f32>>,
}

impl Ivy {
    pub fn new() -> Ivy {
        Ivy {
            root: Vine::new().seed().grow(10),
            colliders: vec![],
        }
    }
}

// VINE
#[derive(Debug)]
pub struct Vine {
    pub settings: VineSettings,
    pub nodes: Vec<VineNode>,
    // pub parent: Option<&'a Vine>,
    pub origin: Point3<f32>,
    children: Vec<Vine>,
}

impl Vine {
    pub fn new() -> Vine {
        Vine {
            settings: VineSettings::new(30, 0.1, 1.5, 0.5, 1.0),
            nodes: vec![],
            origin: Point3::new(0.0, 0.0, 0.0),
            children: vec![],
        }
    }
    pub fn seed(mut self) -> Vine {
        self.nodes = vec![VineNode::new()];
        self.nodes[0].position = self.origin.clone();
        self
    }
    pub fn grow(mut self, count: usize) -> Vine {
        let last_index = self.nodes.len() - 1;
        for i in 0..count {
            // PREVIOUS NODE
            let prev_node = &self.nodes[last_index + i];

            // NEW NODE
            let mut node = VineNode::new();

            node.position = self.nodes[last_index + i].position;
            // GROW VECTOR -> Random + Up + Directionality
            let mut vec_grow = grow_vector(&mut self.settings, prev_node);
            vec_grow = vec_grow.normalize() * self.settings.part_length;
            // FORCES -> Surface Attraction + Gravity

            // COLLISION -> with collider meshes

            // OFFSET -> from colliders

            // SET NODE VALUES -> floating length, sticking since,
            // total length, grow direction, surface direction

            node.position = prev_node.position + vec_grow;
            node.grow_direction = node.position - prev_node.position;

            self.nodes.push(node);
            // last_index += 1;
        }
        self
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

// VINENODE
#[derive(Debug)]
pub struct VineNode {
    pub position: Point3<f32>,
    pub shape: Capsule<f32>,
    pub length: Option<f32>,
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
            length: None,
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
