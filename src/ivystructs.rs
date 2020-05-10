use nalgebra::{Point3, Vector3};
use ncollide3d::shape::{Capsule, TriMesh};
use oorandom::Rand32;

use crate::random_walk::grow_vector;

// IVY
// #[derive(Copy)]
pub struct Ivy {
    // pub settings: IvySettings,
    pub root: Vine,
    pub vines: Vec<Vine>,
    pub colliders: Vec<TriMesh<f32>>,
    pub rng: oorandom::Rand32,
}

impl Ivy {
    pub fn new() -> Ivy {
        Ivy {
            root: Vine::new(0, &Point3::new(0.0, 0.0, 0.0)).seed().grow(10),
            vines: vec![Vine::new(0, &Point3::new(0.0, 0.0, 0.0)).seed().grow(10)],
            colliders: vec![],
            rng: oorandom::Rand32::new(1),
        }
    }
    pub fn grow_vines(mut self, from_level: usize) {
        let vine_count = self.vines.len();
        let mut finished = 0;
        let mut current_depth = from_level;
        while finished < vine_count {
            for vine in &mut self.vines.iter() {
                if vine.depth == current_depth {
                    // &vine.grow(self.rng, 10);
                }
            }
            current_depth += 1;
        }
    }
    pub fn add_vine(mut self, depth: usize, origin: &Point3<f32>, node_count: usize) {
        let mut vine = Vine::new(depth, origin).seed().grow(node_count);
        self.vines.push(vine);
    }
}

// VINE
#[derive(Debug, Clone)]
pub struct Vine {
    pub settings: VineSettings,
    pub nodes: Vec<VineNode>,
    pub origin: Point3<f32>,
    pub children: Vec<Vine>,
    depth: usize,
    max_children: usize,
}

impl Vine {
    pub fn new(depth: usize, origin: &Point3<f32>) -> Vine {
        Vine {
            settings: VineSettings::new(30, 0.1, 1.5, 0.5, 1.0),
            nodes: vec![],
            origin: origin.clone(),
            children: vec![],
            depth,
            max_children: 3,
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
            node.vine_length = prev_node.vine_length + nalgebra::Matrix::norm(&node.grow_direction);

            self.nodes.push(node);
            // TEST FOR SPLIT
            if self.depth < 2 {
                if self.settings.rng.rand_float() < self.settings.child_probability {
                    if self.children.len() <= self.max_children {
                        println!("SEEDING CHILD",);
                        self.children.push(
                            Vine::new(self.depth + 1, &self.nodes[last_index + i].position.clone())
                                .seed()
                                .grow(10),
                        );
                    }
                }
            }
        }
        self
    }
    pub fn regrow(mut self) -> Vine {
        let count = self.nodes.len();
        let start = self.nodes[0].clone();
        self.nodes.clear();
        self.nodes.push(start);
        self.grow(count - 1)
    }
}

#[derive(Debug, Clone)]
pub struct VineSettings {
    pub rng: Rand32,
    pub count: usize,
    pub child_probability: f32,
    pub part_length: f32,
    pub weight_random: f32,
    pub weight_up: f32,
    pub weight_direction: f32,
}

impl VineSettings {
    pub fn new(
        // rng: Rand32,
        count: usize,
        // child_probability: usize,
        part_length: f32,
        weight_random: f32,
        weight_up: f32,
        weight_direction: f32,
    ) -> VineSettings {
        VineSettings {
            rng: Rand32::new_inc(1, 2),
            child_probability: 0.1,
            count,
            part_length,
            weight_random,
            weight_up,
            weight_direction,
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
