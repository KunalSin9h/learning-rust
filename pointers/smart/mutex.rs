use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let x = Arc::new(Mutex::new(0));
    let mut tasks = Vec::new();

    for _ in 0..1000 {
        let y = x.clone();
        let ht = thread::spawn(move || {
            let mut ym = y.lock().unwrap();
            *ym = *ym + 1;
        });
        tasks.push(ht);
    }

    for t in tasks {
        let _ = t.join();
    }

    println!("{:?}", *x.lock().unwrap());
}
