use std::sync::Arc;
use std::thread;

fn main() {
    let owner = Arc::new(0);

    for _ in 0..10 {
        let r = owner.clone();

        let ht = thread::spawn(move || {
            println!("{}", r);
        });

        let _ = ht.join(); // i will wait here until the ht thread will finish
    }

}
