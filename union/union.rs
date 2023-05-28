union MyUnion {
    x: i32,
    y: f32,
}

fn main() {
    let mut u = MyUnion { x: 10 };

    unsafe {
        println!("{}", u.x); // access to union field is unsafe
        println!("{}", u.y);
        u.y = 50.0;
        println!("{}", u.x);
        println!("{}", u.y);
    }
}
/*
 * Output
 *
 * 10
 * 0.000000000000000000000000000000000000000000014
 * 1112014848
 * 50
 *
 *
 * Unpredictable behavior
 */
