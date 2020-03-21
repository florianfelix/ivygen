#![allow(dead_code)]
#![allow(unused_imports)]

mod display;
mod random_walk;
mod utils_r;
mod vine;


fn main() {
    let ivy = vine::Ivy::new();

    // WINDOW SETUP
    display::setup(ivy);
    
}
