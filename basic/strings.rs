fn main(){
    println!("Hello, World");

    let greeting = "Hello";
    let subject = "World";

    println!("{}, {}!", greeting, subject);

    let greet = format!("Hey i am saying, {} {}!", greeting, subject);

    println!("{}", greet);

    panic!("Program will stop now");
}
