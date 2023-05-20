use std::sync::{mpsc, Arc, Mutex};
// use std::thread;

fn main() {
    let (sender, receiver) = mpsc::channel();
    let gv = Arc::new(Mutex::new(0));
    let hugevec = vec![0; 100];
    let mut newvec = vec![];
    let threads = Arc::new(Mutex::new(Vec::new()));
    let mut thread_parallel = vec![];

    for i in 0..10 {
        let sender_clone = sender.clone();
        let gv_clone = gv.clone();
        let threads_clone = threads.clone();
        let mut work: Vec<u32> = Vec::with_capacity(hugevec.len() / 1); // new vec to put the work in. 1/10th the size
        work.extend(&hugevec[i*10..(i+1)*10]); // first part gets 0..100, next gets 100..200, etc.
        let handle = std::thread::spawn(move || { // make a handle

            for number in work.iter_mut() { // do the actual work
                let mut val = gv_clone.lock().unwrap();                                            
                let mut thread_vec = threads_clone.lock().unwrap();
                *number = *val;
                thread_vec.push(val.clone());
                *val += 1;
            };
            sender_clone.send(work).unwrap(); // use the sender_clone to send the work to the receiver
        });
        thread_parallel.push(handle);
    }

    for _ in thread_parallel {
        newvec.push(receiver.try_recv().unwrap())
    }

    // Now we have a Vec<Vec<u32>>. To put it together we can use .flatten()
    let newvec = newvec.into_iter().flatten().collect::<Vec<u32>>(); // Now it's one vec of 1000 u32 numbers
    println!("{:?}", newvec);                                                                    
    println!("---------------------------");
    println!("{:?}", threads.lock().unwrap());                                                                    


    // let (sender, receiver) = channel::<i32>();
    // let sender_clone = sender.clone();
    // let mut threads = vec![];
    // let mut results = vec![];

    // threads.push(thread::spawn(move || {
    //     sender.send(1).unwrap();
    // }));

    // threads.push(thread::spawn(move || {
    //     sender_clone.send(2).unwrap();
    // }));

    // for _ in threads { // Parallel + Finish
    //     results.push(receiver.recv().unwrap());
    // }

    // println!("{:?}", results);
}
