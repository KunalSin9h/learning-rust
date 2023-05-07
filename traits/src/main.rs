mod shapes; // first checks for shapes.rs and upon failure, it checks for shapes/mod.rs
use crate::shapes::{circle::Circle, rectangle::Rectangle};

fn main(){
    let rect = Rectangle::default();
    let c = Circle::default();

    // println!("{}", rect.area()); // i am using something from trait, so i need the Trait Area in
    println!("{}", rect);
    println!("{}", c);
}

