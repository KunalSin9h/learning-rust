mod shapes; // first checks for shapes.rs and upon failure, it checks for shapes/mod.rs
// use crate::shapes::{circle::Circle, rectangle::Rectangle};
 use crate::shapes::rectangle::Rectangle;

fn main(){
    let rect = Rectangle::default(); // possible due because  we can implemented default trait

    // consuming a reference, not the actual value
    for point in &rect { // possible due because  we can implemented into_iter trait
        println!("{:?}", point);
    }

    println!("{}", rect); // display 

    // let c = Circle::default();

    // // println!("{}", rect.area()); // i am using something from trait, so i need the Trait Area in
    // println!("{}", rect);
    // println!("{}", c);


}

