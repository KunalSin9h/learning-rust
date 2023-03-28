/*
 * Ownership
 * Auto Memory Management 
 *
 * Ownership + Bowering + Lifetime
 *
 * Ownership means who's responsibility is to deallocate the memory
 */

fn get_years() -> Vec<u16> {
    let years: Vec<u16> = vec![2002, 2003, 2004, 2005]; // alloc
    return years;  // transfer ownership to Main = Yes
} //dealloc(years) = No
  
fn print_years(years: Vec<u16>) {
    for year in years { // ownership moved to for loop
        print!("{} ", year);
    } // dealloc(years) = Yes
    println!();
}
 
fn main(){
    let years: Vec<u16> = get_years();
    print_years(years.clone()); // performance dec

    for y in years { // value used after move : Error
        println!("{}", y);
    }
    
} // dealloc(years) = No
