fn main() -> () {
    //      void

    let mut point: (i64, i64, i64) = (0, 0, 0);

    point.0 = 3;
    point.1 = 4;
    point.2 = 5;

    let x: i64 = point.0;
    let y: i64 = point.1;
    let z: i64 = point.2;

    let (p, q, r): (i64, i64, i64) = point;

    println!("{}, {}, {}", x, y, z);
    println!("{}, {}, {}", p, q, r);

    let mut x: u32; // definition
    x = 100; // initialization
    println!("{}", x); // use
    x = 10;
    println!("{}", x);

    /*
     * Unit: A Unit is an empty tuple, means a void
     * a () <unit> is a rust version of Void
     * Rust does not have a void keyword
     */

    // let unit: () = ();

    print_hello("abced");

}

fn print_hello(text: &str) -> () {
    println!("{}", text);
}

/*
 * Before use/mutate -> definition + initialization
 * definition and initialization does not need to be at same place
 */
