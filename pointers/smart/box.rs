use std::mem::size_of_val;

trait Vehicle {
    fn drive(&self);
}

struct Truck;

impl Vehicle for Truck {
    fn drive(&self) {
        println!("this is truck"); 
    }
}

fn main() {
    let v: Box<dyn Vehicle>;
    v = Box::new(Truck);
    v.drive();
}
