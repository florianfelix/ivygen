#![allow(dead_code)]
#![allow(unused_imports)]

mod display;
mod random_walk;
mod utils_r;
mod ivystructs;

mod vine;
mod context;


fn main() {
    let ivy = ivystructs::Ivy::new();

    // WINDOW SETUP
    display::setup(ivy);
    
}
