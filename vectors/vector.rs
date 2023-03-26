/*
 Vectors is a dynamic array, which can grow or shrink in size.
 Here Vec<T> i a generic type, where T is a type parameter.
 */

fn main(){

    let mut years: Vec<i32> = vec![2002, 2003, 2004]; // macro called with []

    years.push(2010);
    years.push(2020);

    println!("Number of years in years is {}", years.len());

    for year in years {
        println!("{}", year);
    }

    let arr: [i32; 3] = [-100, 0, 100];

    for a in arr {
        println!("{}", a);
    }


} 
