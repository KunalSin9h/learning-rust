use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(10, Rc::new(Cons(20, Rc::new(Nil)))));
    let b = Cons(1, a.clone()); // 1
    println!("{}", Rc::strong_count(&a)); // 2 -> 1
    let c = Cons(2, a.clone()); // 2
    println!("{}", Rc::strong_count(&a)); // 3
    println!("{}", Rc::strong_count(&a)); // 3
    println!("{}", Rc::strong_count(&a)); // 3
    println!("{}", Rc::strong_count(&a)); // 3
    println!("{}", Rc::strong_count(&a)); // 3
    println!("{}", Rc::strong_count(&a)); // 3
    println!("{}", Rc::strong_count(&a)); // 3
    println!("{}", Rc::strong_count(&a)); // 3
    println!("{}", Rc::strong_count(&a)); // 3
    println!("{}", Rc::strong_count(&a)); // 3
    println!("{}", Rc::strong_count(&a)); // 3
    println!("{}", Rc::strong_count(&a)); // 3
    println!("{}", Rc::strong_count(&a)); // 3
}


/*
 * Rc<T> -> Non-Atomic Reference Counted
 *      This can lead to Data Races when used in Multi Thread Scenario
 *      Thats why it is not recommended to use Rc<T> in concurrent code
 *
 * ARC -> Atomically Reference Counted
 */


