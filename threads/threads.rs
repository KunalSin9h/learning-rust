use std::thread;
use std::time::Duration;

const OFF: &str = "\x1b[0m";
const RED: &str = "\x1b[1;31m";
const GREEN: &str = "\x1b[1;32m";
const YELLOW: &str = "\x1b[1;33m";

// static mut INC: usize = 0;

// fn inc() -> &'static usize {
//     unsafe {
//         INC += 1;
//         return &INC;
//     }
// }

fn main(){

    // let arr: Vec<usize> = vec![1, 2, 3];

    let handle_first = thread::spawn(move || {
        loop  {
            println!("{} {} {}", RED, "RED",  OFF);
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    let handle_second = thread::spawn(move || {
        loop  {
            println!("{} {} {}", GREEN, "****GREEN",  OFF);
            thread::sleep(Duration::from_millis(200));
        }
    });

    let handle_third = thread::spawn( move || {
        loop  {
            println!("{} {} {}", YELLOW, "========BLUE",  OFF);
            thread::sleep(Duration::from_millis(300));
        }
    });
    
    handle_first.join().unwrap();
    handle_second.join().unwrap();
    handle_third.join().unwrap();
}
