use std::io::stdin;

fn main(){

    let mut first_name = String::new();
    let mut last_name = String::new();

    stdin().read_line(&mut first_name).unwrap();
    stdin().read_line(&mut last_name).unwrap();

    let x: Vec<_> = first_name.split(" ").collect();

    dbg!(&first_name);
    dbg!(&last_name);
    dbg!(&x);

    println!("{} {}", first_name, last_name);

}
