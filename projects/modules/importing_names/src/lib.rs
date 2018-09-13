
pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

fn main() {
    a::series::of::nested_modules();
}

// with use

use a::series::of;

fn main() {
    of::nested_modules();
}

// enums also form a sort of namespace like modules, we can import and enum's
// variants with use as well.

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::{Red, Yellow};

fn main(){
    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;
}

// Glob Imports with *
use TrafficLight::*;

fn main(){
    let red = Red;
    let yellow = Yellow;
    let green = Green;
}
