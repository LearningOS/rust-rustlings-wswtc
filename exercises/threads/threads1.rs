// threads1.rs
// Execute `rustlings hint threads1` or use the `hint` watch subcommand for a hint.
// This program should wait until all the spawned threads have finished before exiting.



use std::thread;
use std::time::Duration;


fn main() {

    let mut handles = vec![];
    for i in 0..10 {
        handles.push(thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            println!("thread {} is complete", i);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("All threads completed successfully!");
    
}
