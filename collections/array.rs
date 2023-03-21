fn main(){

    let my_arr = [1, 2, 3, 4, 5];


    for item in my_arr.iter() {
        println!("{}", item);
    }

    println!();

    let mut index = 0;
    while index < my_arr.len() {
        println!("{}", my_arr[index]);
        index += 1;
    }

    print!("\n");

    for (idx, val) in my_arr.iter().enumerate() {
        println!("The value at index {} is {}", idx, val);
    }

    print!("\n");


    let grades: [&str; 5] = ["B", "C+", "B+", "B", "B"];

    println!("{}", grades[0]);
    println!("{}", grades[1]);
    println!("{}", grades[2]);
    println!("{}", grades[3]);
    println!("{}", grades[4]);

    println!("all grades..");


    for grade  in grades.iter(){
        println!("The grade is {}", grade);
    }
}
