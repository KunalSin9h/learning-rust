use std::env;

fn main(){
    for x in env::vars() {
        println!("Key: {} | value: {}", x.0, x.1);
    }
    
    // println!("{}", env::var("DB").expect("Failed to get var DB"));

    println!("{}", env::var("DB").unwrap_or("https://local:local@localhsot:5656".into())); // runtime
    println!("{}", env!("DB_COMPILE_TIME")); // compile time
}
