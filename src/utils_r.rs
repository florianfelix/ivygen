use nalgebra::Vector3;

pub fn random_positive_unit_vector() -> Vector3<f32> {
    // let v: Vector3<f32> = Vector3::new_random();
    // println!("mag: {}", v.magnitude());
    
    Vector3::new_random().normalize()
}

pub fn random_unit_vector() -> Vector3<f32> {
    let mut v: Vector3<f32> = Vector3::new_random();
    v = v.normalize();
    v -= Vector3::new(0.5, 0.5, 0.5);
    v = v.normalize();
    // println!("mag: {}", v.magnitude());
    v
}