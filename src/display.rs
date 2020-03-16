use kiss3d::light::Light;
use kiss3d::window::Window;
use nalgebra::{Point3, Translation3, UnitQuaternion, Vector3};
use std::path::Path;

pub fn setup() -> Window {
    println!("Setup");
    let mut window = Window::new("Kiss3d: cube");
    window.set_title("Test Cube");
    window.set_point_size(6.0);

    // let mut c      = window.add_cube(1.0, 1.0, 1.0);
    // let mut monkey = window.add_obj(
    //     Path::new("./models/monkey.obj"),
    //     Path::new("./models"),
    //     Vector3::new(1.0, 1.0, 1.0),
    // );
    // // monkey.set_local_translation(Translation3::new(0.0, 2.0, 0.0));
    // monkey.set_color(1.0, 0.0, 0.0);

    // let lpos = Point3::new(1.0, 1.0, 0.0);
    // window.set_light(Light::Absolute(lpos));

    // let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);

    window
}
