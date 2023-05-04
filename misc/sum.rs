use std::collections::HashMap;

fn main(){

    vec![0, 2, 5, 6, 7, 8, 1, 2, 3]
        // .iter()
        // .skip(2)
        // .take_while(|&&x| x > 4)
        // .for_each(|x| print!("{} ", x));
        

        .iter()
        .filter(|x| *x % 2 == 0) // iter() is reference iterator so, i have to de reference
        .for_each(|x| print!("{} ", x));

    println!();

    let map = HashMap::from([
        ("foo", 1),
        ("bar", 2),
        ("baz", 3),
    ]);

    map
        .iter()
        .for_each(|(k, v)| println!("Key: {}, Val: {}", k, v));
}
