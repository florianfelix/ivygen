use nalgebra::Vector3;
// use oorandom;

pub fn random_positive_unit_vector() -> Vector3<f32> {
    // let v: Vector3<f32> = Vector3::new_random();
    // println!("mag: {}", v.magnitude());
    Vector3::new_random().normalize()
}

pub fn random_unit_vector(rng: &mut oorandom::Rand32) -> Vector3<f32> {
    Vector3::new(
        rng.rand_float() - 0.5,
        rng.rand_float() - 0.5,
        rng.rand_float() - 0.5,
    )
    .normalize()
}
