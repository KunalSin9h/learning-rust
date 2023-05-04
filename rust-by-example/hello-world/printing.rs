fn main(){
    println!("hello {1}, {0}, {0}", "A", "B");
    println!("hello {first_name}, {last_name}", first_name="Kunal", last_name="Singh");
    println!("Base 10      {}", 1000);
    println!("Base 2      {:b}", 1000);
    println!("Base 8      {:o}", 1000);
    println!("Base 16      {:x}", 1000);
    println!("Base 16X     {:X}", 1000);

    // convert a number in to binary
    let bin = format!("{:b}", 100);
    println!("{}", bin);

    // formatting
    println!("{number:>5}", number=10);
    println!("{number:>5}", number=100);
    println!("{number:>5}", number=1000);
    println!("{number:>5}", number=10000);
    print!("hello");
}
