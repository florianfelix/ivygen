use nalgebra::{Point3, Vector3};

use crate::utils_r::{random_unit_vector};

pub fn random_walk() -> Vec<Point3<f32>> {
    let mut result: Vec<Point3<f32>> = vec![Point3::new(0.0, 0.0, 0.0)];

    for i in 0..30 {
        let new = result[i] + random_unit_vector();
        result.push(new);
    }

    result
}

pub fn vine(count: usize) -> Vec<Point3<f32>> {
    let mut vine: Vec<Point3<f32>> = vec![Point3::new(0.0, 0.0, 0.0)];

    // let count = 30;
    let part_length = 0.1;
    let v_up = Vector3::new(0.0, 1.0, 0.0);

    let random_weight = 1.0;
    let up_weight = 0.0;
    let direc_weight = 0.0;

    for i in 0..count {
        // RANDOM
        let mut v = random_unit_vector() * random_weight;

        // UP
        v = v + v_up * up_weight;

        // DIRECTIONALITY
        if i > 0 {
            let direc = vine[i] - vine[i - 1];
            v = v + direc.normalize() * direc_weight;
        }
        // FORCES

        // COLLISION

        // PART LENGTH
        v = v.normalize() * part_length;

        // CONSTRUCT POINT
        let p = vine[i] + v;
        vine.push(p);
    }

    vine
}
