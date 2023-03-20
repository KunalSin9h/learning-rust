fn multiply_both(x: f64, y: f64) -> f64 {
    return x * y;
}

fn main(){

    let ans: f64 = multiply_both(1.1, 2.3);
    println!("1.1 * 2.3 = {}", ans);

    let x: f64 = 10.0 / 3.0;
    let y: f32 = 10.0 / 3.0;

    println!("f64 -> {}", x);
    println!("f32 -> {}", y);


    println!("{}", 5.0 / 0.0);

}
