fn main(){
    let x: i32 = 10;
    let mut y = &x; 
    println!("{}", y);
    y = &100;
    println!("{}", x);
    println!("{}", y);

    let foo: () = if true {
        println!("Hello");
    }; // unit
}
