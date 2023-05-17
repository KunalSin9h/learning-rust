use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main(){
    let x = Arc::new(Mutex::new(0));

    for _ in 0..1000 {
        let y = x.clone();
        let ht = thread::spawn(move || {
            let mut ym = y.lock().unwrap(); // i am going to acquire, means no thread can use it
            *ym = *ym + 1;                                            
        });
        let _ = ht.join();
    }
    thread::sleep(Duration::from_secs(1));
    println!("{:?}", x);
}
