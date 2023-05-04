fn not_complete_fn(x: usize) -> bool {
    if x < 10 {
        return true;
    }
    // todo!("Need to finish this");
    unimplemented!("Not going to finish this");
}

fn only_evens(x: usize) -> bool {
    if x % 2 == 0 {
        return true;
    } else {
        unreachable!("You fucked up!");
    }
}

fn main(){

    let x = not_complete_fn(1);
    println!("{}", x);

    let y = only_evens(2);
    println!("{}", y);

    // unwrap will extract values from Options<T> and Result<O, E>
    let foo = Some(String::from("Hello")).unwrap();
    println!("{}", foo);
}
