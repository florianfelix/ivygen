#![allow(dead_code)]
#![allow(unused_imports)]

// use nalgebra::Point3;
// use kiss3d::window::Window;
use nalgebra::Point3;
mod display;
mod random_walk;
mod utils_r;
mod vine;

fn main() {
    let points = random_walk::vine();
    // println!("{:?}", points);

    let mut window = display::setup();
    window.set_point_size(6.0);

    // HARDCODED VARS
    let origin = Point3::new(0.0, 0.0, 0.0);
    let xa = Point3::new(1.0, 0.0, 0.0);
    let ya = Point3::new(0.0, 1.0, 0.0);
    let za = Point3::new(0.0, 0.0, 1.0);


    while window.render() {
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