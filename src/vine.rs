
use oorandom::Rand32;
use nalgebra::{Point3, Vector3};
use ncollide3d::shape::{Capsule, TriMesh};

use crate::context::Context;
use crate::utils_r::random_unit_vector;

#[derive(Debug, Clone)]
pub struct Vine <'a> {
    pub context: &'a Context,
    pub settings: VineSettings,
    pub rng: Rand32,
    pub nodes: Vec<VineNode>,
    pub origin: Point3<f32>,
    pub max_nodes: usize,
    id: usize,
    depth: usize,
    children_seeded: usize,
}

impl<'a> Vine<'a> {
    pub fn new(context: &'a Context, depth: usize, origin: &Point3<f32>, id: usize) -> Vine<'a> {
        let mut seed_node = VineNode::new();
        seed_node.position = origin.clone();
        Vine {
            context,
            settings: VineSettings::new(30, 0.1, 1.5, 0.5, 1.0),
            depth,
            origin: origin.clone(),
            // nodes: vec![],
            nodes: vec![seed_node],
            max_nodes: 500,
            id,
            children_seeded: 3,
            rng: Rand32::new(0 + context.seed_base)
        }
    }

    // fn random_vector(mut self) -> Vector3<f32> {
    //     random_unit_vector(&mut self.rng)
    // }

    fn grow_vector(rng: &mut oorandom::Rand32, settings: &VineSettings, prev_node: &VineNode) -> Vector3<f32> {
    
        // RANDOM
        let v_rand = random_unit_vector(rng) * settings.weight_random;
        
        // UP
        let v_up = Vector3::new(0.0, 1.0, 0.0) * settings.weight_up;
        // v = v + v_up * settings.weight_up;
    
        // DIRECTIONALITY
        let v_direction = prev_node.grow_direction.normalize() * settings.weight_direction;
        
        v_rand + v_up + v_direction
    }

    pub fn grow(mut self, count: usize) {
        let last_index = self.nodes.len() - 1;
        for i in 0..count {
            // PREVIOUS NODE
            let prev_node = &self.nodes[last_index + i];

            // NEW NODE
            let mut node = VineNode::new();

            node.position = self.nodes[last_index + i].position;
            // GROW VECTOR -> Random + Up + Directionality
            let mut vec_grow = Vine::grow_vector(&mut self.rng, &self.settings, prev_node);
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
            // if self.depth < 2 {
            //     if self.settings.rng.rand_float() < self.settings.child_probability {
            //         if self.children.len() <= self.max_children {
            //             println!("SEEDING CHILD",);
            //             self.children.push(
            //                 Vine::new(self.depth + 1, &self.nodes[last_index + i].position.clone())
            //                     .seed()
            //                     .grow(10),
            //             );
            //         }
            //     }
            // }
        }
        // self
    }
}

#[derive(Debug, Clone)]
pub struct VineSettings {
    // pub rng: Rand32,
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
            // rng: Rand32::new_inc(1, 2),
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