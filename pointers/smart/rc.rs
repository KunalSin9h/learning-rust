use std::rc::Rc;

#[derive(Debug)]
struct Truck {
    capacity: i32,
}

fn main() {

    let (
        truck_a, 
        truck_b, 
        truck_c
    ) = (
        Rc::new(Truck { capacity: 1 }), 
        Rc::new(Truck { capacity: 2 }), 
        Rc::new(Truck { capacity: 3 }),
    );

    let facility_one = vec![truck_a, truck_b.clone()];
    let facility_two = vec![truck_b.clone(), truck_c];
    
    println!("{:?}", facility_one);
    println!("{:?}", facility_two);

    println!("{:?}", Rc::strong_count(&truck_b.clone()));
    std::mem::drop(facility_two);
    println!("{:?}", Rc::strong_count(&truck_b));
}
