fn main(){
    /*
     *
     * Closure in TS
     * (x) => {
     *  return x + 1;
     * }
     *
     * (x) => x + 1;
     *
     * In Rust 
     *
     * |x| {
     *  return x + 1;
     * }
     *
     *  |x| x + 1;
     *
     */
    
    let mut v = vec![1, 2, 3, 4, 5];
    v.push(10);
    v.push(100);
    v.pop();
    for i in &v {
        print!("{} ", i);
    }
    println!();
    println!("{}", v[0]);
    println!("{}", v[1]);
    println!("{}", v[2]);
    println!("{}", v[3]);
    println!("{}", v[4]);
    println!("{}", v[5]);

    let x: Option<&i32> = v.get(10);
    match x {
        None => println!("Index out of bound"),
        Some(value) => {
            println!("The value is {}", value);
        }
    }

    let tuple = (1, 2, 3);
    println!("{}", tuple.0);
    println!("{}", tuple.1);
    println!("{}", tuple.2);

}
