fn main()  {
    let x = 10;
    let y: *const i32 = &x;
    println!("{:?}", y);
}
