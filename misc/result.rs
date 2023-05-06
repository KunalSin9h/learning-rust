fn give_error() -> Result<i32, &'static str> {
    println!("I am here");
    return Err("Hello this failed");
}

fn main(){

    _ = give_error(); // ignore the error return

    // ignore error from a function
    // give_error().expect("panicked bro"); // expect means this program is not going to give
                                              // error but if it does then panic me

    let foo = Err("error this is").unwrap_or(10);
    dbg!(foo);
}
