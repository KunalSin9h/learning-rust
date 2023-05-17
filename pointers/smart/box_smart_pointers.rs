use std::fmt::Display;
/*
 * Recursive enum
 */
// linked list
enum List {
    Node(i32, Box<List>), // ----\ ____> Enum Variants
    Nil, // ----------------/
}

impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let x =  match self {
                Node(x, _) => {
                    x
                }
                Nil => {
                    &-1
                }
            };

        return write!(f, "{}", x);
    }
}

use List::{Node, Nil};

fn main(){
//     let b: Box<i32> = Box::new(10);
//     dbg!(b);
//

    let my_list = Node(10, Box::new(Node(2, Box::new(Nil))));
    println!("{}", my_list);
}
