// Worker Pool

// use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::{Arc, Mutex};
use std::thread;

static N_THREADS: u32 = 40;

fn main(){

    let queue = Arc::new(Mutex::new(Vec::new()));
    let res = Arc::new(Mutex::new(Vec::new()));
    let mut threads = vec![];

    for i in 1..100 {
        queue.lock().unwrap().push(i as u32);
    }

    queue.lock().unwrap().reverse();

    for _ in 0..N_THREADS {
        let queue_clone = queue.clone();
        let res_clone = res.clone();

        let x = thread::spawn(move || {
            worker(&queue_clone, &res_clone);
        });

        threads.push(x);
    }

    for thread in threads {
        thread.join().unwrap();
    }


    println!("{:?}", res.lock().unwrap());

}

fn worker(queue: &Arc<Mutex<Vec<u32>>>, res: &Arc<Mutex<Vec<u128>>>) {
    while let Some(val) = queue.lock().unwrap().pop() {
        let v = fib(val);
        res.lock().unwrap().push(v); 
        println!("{}", v);
    }
}

fn fib(number: u32) -> u128 {
    if number <= 1 {
        return number as u128;
    }
    return fib(number - 1) + fib(number - 2); 
}
