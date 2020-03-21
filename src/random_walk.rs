use nalgebra::{Point3, Vector3};

use crate::utils_r::random_unit_vector;
use crate::vine::VineSettings;
use crate::vine::{VineNode};

pub fn grow_vector(settings: &mut VineSettings, prev_node: &VineNode) -> Vector3<f32> {
    
    // RANDOM
    let v_rand = random_unit_vector(&mut settings.rng) * settings.weight_random;
    
    // UP
    let v_up = Vector3::new(0.0, 1.0, 0.0) * settings.weight_up;
    // v = v + v_up * settings.weight_up;

    // DIRECTIONALITY
    let v_direction = prev_node.grow_direction.normalize() * settings.weight_direction;
    
    v_rand + v_up + v_direction
}

pub fn vine(settings: &mut VineSettings) -> Vec<Point3<f32>> {
    // START VINE AT ORIGIN
    let mut vine: Vec<Point3<f32>> = vec![Point3::new(0.0, 0.0, 0.0)];

    let v_up = Vector3::new(0.0, 1.0, 0.0);

    for i in 0..settings.count {
        // RANDOM
        let mut v = random_unit_vector(&mut settings.rng) * settings.weight_random;

        // UP
        v = v + v_up * settings.weight_up;

        // DIRECTIONALITY
        if i > 0 {
            let direc = vine[i] - vine[i - 1];
            v = v + direc.normalize() * settings.weight_direction;
        }
        // FORCES

        // COLLISION

        // PART LENGTH
        v = v.normalize() * settings.part_length;

        // CONSTRUCT POINT
        let p = vine[i] + v;
        vine.push(p);
    }

    vine
}
