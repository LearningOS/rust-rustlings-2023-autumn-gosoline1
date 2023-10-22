// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.



use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: Arc<Mutex<mpsc::Sender<u32>>>) -> () {
    let qc = Arc::new(q);
    let qc1 = Arc::clone(&qc);
    let qc2 = Arc::clone(&qc);
    
    let tx1 = Arc::clone(&tx);
    let tx2 = Arc::clone(&tx);

    let first_thread = thread::spawn(move || {
        for val in &qc1.first_half {
            println!("sending {:?}", val);
            let sender = tx1.lock().unwrap();
            sender.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let second_thread = thread::spawn(move || {
        for val in &qc2.second_half {
            println!("sending {:?}", val);
            let sender = tx2.lock().unwrap();
            sender.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    first_thread.join().unwrap();
    second_thread.join().unwrap();
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();
    let queue_length = queue.length;
    let tx = Arc::new(Mutex::new(tx)); // 包装 Sender 以供多个线程共享

    send_tx(queue, Arc::clone(&tx));

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
        if total_received == queue_length {
            break;
        }
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length)
}