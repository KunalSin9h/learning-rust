/*
Vectors is a dynamic array, which can grow or shrink in size.
Here Vec<T> i a generic type, where T is a type parameter.
*/

fn main() {
    let mut years: Vec<i32> = vec![2002, 2003, 2004]; // macro called with []

    years.push(2010);
    years.push(2020);

    let total_years = years.len();

    println!("Number of years in years is {}", total_years);

    for year in years {
        println!("{}", year);
    }

    let arr: [i32; 3] = [-100, 0, 100];

    for a in arr {
        println!("{}", a);
    }

    let mut numbs: Vec<i32> = Vec::with_capacity(10);
    numbs.push(1);
    numbs.push(2);
    numbs.push(3);
    numbs.push(4);
    numbs.push(5);

    let numbs_size: usize = numbs.len();
    let numbs_capacity: usize = numbs.capacity();

    println!("The Numbs length is {} and the numbs capacity is {}", numbs_size, numbs_capacity);

}
