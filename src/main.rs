#![allow(dead_code)]
#![allow(unused_imports)]

mod display;
mod random_walk;
mod utils_r;
mod vine;

// use nalgebra::Point3;
// use kiss3d::window::Window;
use kiss3d::event::{Action, Key, Modifiers, WindowEvent};
use kiss3d::text::{Font, TextRenderer};
use kiss3d::window::Window;
use nalgebra::{Point2, Point3};
use oorandom;
use std::path::Path;

fn main() {
    // BASE VINE
    let mut vine_settings = vine::VineSettings::new(30, 0.1, 1.5, 0.5, 1.0);
    let mut points = random_walk::vine(&mut vine_settings);
    // WINDOW SETUP
    let mut window = display::setup();

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

    while window.render() {
        for event in window.events().iter() {
            match event.value {
                WindowEvent::Key(key, Action::Press, _) => {
                    if key == Key::A {
                        window.draw_text(
                            &vine_settings.count.to_string(),
                            &Point2::new(30.0, 330.0),
                            40.0,
                            &font,
                            &Point3::new(1.0, 1.0, 1.0),
                        );
                    } else if key == Key::Space {
                        points = random_walk::vine(&mut vine_settings);
                    } else if key == Key::PageUp {
                        vine_settings.count += 10;
                        points = random_walk::vine(&mut vine_settings);
                    } else if key == Key::PageDown {
                        if vine_settings.count > 11 {
                            vine_settings.count -= 10;
                        }
                        points = random_walk::vine(&mut vine_settings);
                    }
                }
                WindowEvent::Scroll(n1, n2, Modifiers::Shift) => println!("{} {}", n1, n2),
                _ => {}
            }
        }

        // RENDER VINES AS POINTS AND LINES
        for (i, p) in points.iter().enumerate() {
            window.draw_point(&p, &Point3::new(1.0, 0.0, 0.0));
            if i > 0 {
                window.draw_line(&points[i - 1], &points[i], &Point3::new(1.0, 1.0, 1.0));
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
            40.0,
            &font,
            &Point3::new(1.0, 1.0, 1.0),
        );
    }
}
