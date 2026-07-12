use std::thread;
use std::rc::Rc;
// use std::sync::mpsc;
//use std::sync::{Arc, Mutex};

fn main() {

    let data = Rc::new(5);

    thread::spawn(move || {
        println!("{data}");
    }).join().unwrap();

    /* 
    let total = Arc::new(Mutex::new(0));
    let mut workers = vec![];

    for i in 1..=5 {
        let total = Arc::clone(&total);
        let worker = thread::spawn(move || {
            let mut sum = total.lock().unwrap();
            *sum += i;
        });
        workers.push(worker);
    }

    for worker in workers {
        worker.join().unwrap();
    }
    println!("total: {}", *total.lock().unwrap());

    
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(String::from("hello")).unwrap();
    });

    let recieved = rx.recv().unwrap();
    println!("got: {recieved}");

    
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("worker: {i}");
        }
    });
    

    let names = vec![String::from("ada"), String::from("will")];

    let handle = thread::spawn(move || {
        println!("{names:?}"); // names is now owned by the thread
    });

    handle.join().unwrap();
    */
}
