fn multiply(x: f64, y: u8) -> f64 {
    x * (y as f64)
    // can not multiply f64 * u8
}

fn main() {
    let x: i32 = 100;
    println!("{}", x);

    let ans: f64 = multiply(4.5, 2);

    println!("{}", ans);
}
