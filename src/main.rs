#![allow(dead_code)]
#![allow(unused_imports)]

// use nalgebra::Point3;
// use kiss3d::window::Window;
use kiss3d::window::Window;
use kiss3d::event::{Action, Key, WindowEvent, Modifiers};
use nalgebra::Point3;
mod display;
mod random_walk;
mod utils_r;
mod vine;

fn main() {
    let mut count: usize = 30;
    let mut points = random_walk::vine(count);
    // println!("{:?}", points);

    let mut window = display::setup();

    // HARDCODED VARS
    let origin = Point3::new(0.0, 0.0, 0.0);
    let xa = Point3::new(1.0, 0.0, 0.0);
    let ya = Point3::new(0.0, 1.0, 0.0);
    let za = Point3::new(0.0, 0.0, 1.0);

    
    while window.render() {
        for event in window.events().iter() {
            match event.value {
                WindowEvent::Key(key, Action::Press, _) => {
                    if key == Key::A {

                    } else if key == Key::Space {
                        points = random_walk::vine(count);
                    } else if key == Key::PageUp {
                        count += 10;
                        points = random_walk::vine(count);
                    } else if key == Key::PageDown {
                        count -= 10;
                        points = random_walk::vine(count);
                    }
                },
                WindowEvent::Scroll(n1, n2, Modifiers::Shift) => {
                    println!("{} {}", n1, n2)
                }
                _ => {}
            }
        }

        for (i, p) in points.iter().enumerate() {
            window.draw_point(&p, &Point3::new(1.0, 0.0, 0.0));
            if i > 0 {
                window.draw_line(&points[i - 1], &points[i], &Point3::new(1.0, 1.0, 1.0));
            }
        }

        // Axais
        window.draw_line(&origin, &xa, &xa);
        window.draw_line(&origin, &ya, &ya);
        window.draw_line(&origin, &za, &za);


    }
}

// fn draw_lines(&mut w: Window, points: Vec<Point3<f32>>) {
    //     for (i, p) in points.iter().enumerate() {
        //         let len = points.len();
        //         if i < len {
            //             if i > 0 {
                //                 w.draw_line(&points[i-1], &points[i], &Point3::new(1.0, 1.0, 1.0))
                //             }
                //         }
                //     }
                // }
                
// draw_lines(window, rw);
// window.draw_line(&rw[0], &rw[1], &Point3::new(1.0, 1.0, 1.0));
// monkey.prepend_to_local_rotation(&rot);
// window.draw_point(&Point3::new(1.0, 1.0, 0.0), &Point3::new(1.0, 1.0, 1.0));