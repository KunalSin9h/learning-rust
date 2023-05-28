mod meltred {

    type Val = i32; // Type Aliases

    pub fn max(a: i32, b: i32, c: i32) -> Val {
        max_(max_(a, b), c)
    }

    fn max_(a: i32, b: i32) -> Val {
        if a > b {
            return a;
        }
        b
    }

    pub fn add(x: i32, y: i32) -> Val {
        x + y
    }
} // meltred

fn main() {
    type Number = i32; // type aliases

    let x: Number = meltred::add(1, 2);
    println!("{}", x);

    println!("{}", meltred::max(1, 9, 9));
}
