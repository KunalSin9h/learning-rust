/*
 * Type Parameters<T>
 */

enum Option<T>{
    None,
    Some(T),
}

enum Result<O, E> {
    Ok(O),
    Err(E),
}

fn main(){
    let none: Option<char> = Option::None;
    let some: Option<char> = Option::Some('a');

    match none {
        Option::None => println!("This is none"),
        Option::Some(x) => println!("The value is {}", x), 
    }

    match some {
        Option::None => println!("This is none"),
        Option::Some(x) => println!("The value is {}", x), 
    }

    let ok: Result<u64, &str> = Result::Ok(10);
    let err: Result<u64, &str> = Result::Err("Error Reported");

    match ok {
        Result::Ok(o) => println!("The result value is {}", o),
        Result::Err(err) => println!("The error is {}", err),
    }

    match err {
        Result::Ok(o) => println!("The result value is {}", o),
        Result::Err(err) => println!("The error is {}", err),
    }
}
