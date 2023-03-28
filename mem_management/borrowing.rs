/*
 * Referencing
 * Auto Memory Management 
 *
 * Ownership + Bowering + Lifetime
 *
 * Referencing means having ability to access something without taking ownership
 *
 *  -> Vec<i32> => Vector of i32
 *  -> &Vec<i32> => Reference to Vector of i32
 *
 * ⚠️ At same scope we are allowed to have
 *  1. One mutable reference
 *              OR
 *  2. Any number of immutable reference
 */


fn get_years() -> Vec<u16> {
    let years: Vec<u16> = vec![2002, 2003, 2004, 2005]; // alloc
    return years;  // transfer ownership to Main = Yes
} //dealloc(years) = No

/*
 * print_years has decided to use Vec<u16> only for referencing, 
 * it is not going to take ownership
 */
fn print_years(years: &Vec<u16>) { // i can still access but im not the owner
    for year in years.iter() {
        print!("{} ", year);
    } 
    println!();
}// dealloc(years) = NO

fn update_latest_year(years: &mut Vec<u16>) {
    years.push(2023);
    add_future_year(years);
}

fn add_future_year(years: &mut Vec<u16>){
    years.push(2050);
}

fn main(){
    let mut years: Vec<u16> = get_years();

    {
        let year_mut_ref: &mut Vec<u16> = &mut years;
        let year_mut_ref_again: &mut Vec<u16> = &mut years;

        add_future_year(year_mut_ref);
        add_future_year(year_mut_ref_again);
    }

    print_years(&years); // immutable borrow
    update_latest_year(&mut years);   // mutable borrow                       

    println!("Updated year");
    for y in years { // value used after move : Error
        println!("{}", y);
    }
    
} // dealloc(years) = No
