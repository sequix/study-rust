// Send trait, able of transferring ownership between threads (rc not send)
// Sync trait, safe to reference from multiple threads (rc, refcell, cell not)
// types that are made up of Send and Sync traits are automatically also Send and Syn

use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 1..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap()
    }
    println!("Result: {}", *counter.lock().unwrap())
}