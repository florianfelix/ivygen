// use kiss3d::window::Window;
use crate::vine::{Ivy, Vine, VineNode, VineSettings};
use nalgebra::{Point2, Point3, Translation3, UnitQuaternion, Vector3};
use std::path::Path;

use kiss3d::event::{Action, Key, Modifiers, WindowEvent};
use kiss3d::light::Light;
use kiss3d::text::{Font, TextRenderer};
use kiss3d::window::Window;

pub fn setup(mut ivy: Ivy) -> Window {
    // WINDOW SETUP
    let mut window = Window::new("Ivigen");
    window.set_point_size(4.0);
    window.set_background_color(0.1, 0.1, 0.1);

    // CAMERA
    let eye = Point3::new(3.5f32, 1.0, 3.0);
    let at = Point3::origin();
    let mut arc_ball = kiss3d::camera::ArcBall::new(eye, at);

    // HARDCODED VARS
    let origin = Point3::new(0.0, 0.0, 0.0);
    let xa = Point3::new(1.0, 0.0, 0.0);
    let ya = Point3::new(0.0, 1.0, 0.0);
    let za = Point3::new(0.0, 0.0, 1.0);

    // TEXT
    let font = Font::new(Path::new("./fonts/ShareTechMono-Regular.ttf")).unwrap();
    let text = "
    Zoom Out    : MouseWheel Forward
    Zoom In     : MouseWheel Backward
    Rotate      : Left Mouse Drag
    New Vine    : Spacebar
    + 10 Points : PageUp
    - 10 Points : PageDown";

    while window.render_with_camera(&mut arc_ball) {
        for event in window.events().iter() {
            match event.value {
                WindowEvent::Key(key, Action::Press, _) => {
                    if key == Key::A {
                    } else if key == Key::Space {
                        ivy.root = ivy.root.regrow();
                    } else if key == Key::PageUp {
                        ivy.root = ivy.root.grow(10);
                    } else if key == Key::PageDown {
                        if ivy.root.nodes.len() > 11 {
                            ivy.root
                                .nodes
                                .resize(ivy.root.nodes.len() - 10, VineNode::new());
                        } else {
                            ivy.root.nodes.resize(1, VineNode::new());
                        }
                    }
                }
                WindowEvent::Scroll(n1, n2, Modifiers::Shift) => println!("{} {}", n1, n2),
                _ => {}
            }
        }

        for (i, node) in ivy.root.nodes.iter().enumerate() {
            window.draw_point(&node.position, &Point3::new(0.0, 1.0, 0.0));
            if i > 0 {
                window.draw_line(
                    &node.position,
                    &ivy.root.nodes[i - 1].position,
                    &Point3::new(1.0, 1.0, 1.0),
                )
            }
        }

        // DRAW WORLD SPACE AXIS
        window.draw_line(&origin, &xa, &xa);
        window.draw_line(&origin, &ya, &ya);
        window.draw_line(&origin, &za, &za);

        // TEXT RENDER
        window.draw_text(
            text,
            &Point2::new(4.0, 8.0),
            34.0,
            &font,
            &Point3::new(1.0, 1.0, 1.0),
        );
    }

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
