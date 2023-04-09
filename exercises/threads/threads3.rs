// threads3.rs
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a hint.



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

fn send_tx(q: Arc<Queue>, tx: Arc<Mutex<mpsc::Sender<u32>>>) -> () {
    let qc = Arc::clone(&q);
    let tx_shared = Arc::clone(&tx);

    thread::spawn(move || {
        for val in &qc.first_half {
            println!("sending {:?}", val);
            let mut guard = tx_shared.lock().unwrap();
            guard.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let qc = Arc::clone(&q);
    let tx_shared = Arc::clone(&tx);

    thread::spawn(move || {
        for val in &qc.second_half {
            println!("sending {:?}", val);
            let mut guard = tx_shared.lock().unwrap();
            guard.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Arc::new(Queue::new());
    let queue_length = queue.length;

    let tx_shared = Arc::new(Mutex::new(tx));
    let tx_shared1 = Arc::clone(&tx_shared);

    send_tx(Arc::clone(&queue), tx_shared1);

    drop(tx_shared); // Not necessary, but lets us catch some bugs.

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

