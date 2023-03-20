fn main(){

    let is_6_prime: bool = false;
    let is_5_prime: bool = true;

    println!("{}, {}", is_5_prime as u8, is_6_prime as u8);
    println!("{}, {}", is_5_prime, is_6_prime);



    if is_5_prime {
        println!("Yes, 5 is prime");
    } 

    if is_6_prime {
        println!("Yes, 6 is prime");
    } else {
        println!("No, 6 is not prime");
    }


    let var: i32 = 12;

    if var == 10 {
        println!("Yes it is 10");
    } 
    else if var == 11 {
        println!("Yes it is 11");
    } 
    else if var == 12 {
        println!("Yes it is 12");
    } 
    else {
        println!("It is none");
    }

} 
