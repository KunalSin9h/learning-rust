#[macro_export] // procedural macro
macro_rules! hello { // declarative macro
    () => {
        println!("Hello World");
    };
}

fn main(){
    hello!();
}
