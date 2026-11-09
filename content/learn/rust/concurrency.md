+++
title = "Concurrency"
+++

One of the goals of Rust is to make concurrency as safe as can be.

Example:

```rust
use std::sync::mpsc; // Multiple producer, single consumer (mpsc)
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // Transmitter and Receiver
    let (tx, rx) = mpsc::channel();

    // Mutex must be stored an atomic shared pointer to share between threads
    let counter = Arc::new(Mutex::new(0));

    // Threads list
    let mut handles = vec![];

    for i in 0..=10 {
        let tx2 = tx.clone();
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            // Sleep for 1 millisecond
            thread::sleep(Duration::from_millis(100));

            // Will release the lock at the end of the scope
            let mut num = counter.lock().unwrap();
            *num += 1;

            // Transmit string to reciever
            tx2.send(format!("Thread number: {}, Mut number: {}", i, num)).unwrap();
        });
        handles.push(handle);
    }

    // Received will end when all tx go out of scope
    // tx acts like a shared pointer, keeping track of references
    drop(tx);
    for received in rx {
        println!("{}", received);
    }

    // Wait for threads to stop
    for handle in handles {
        handle.join().unwrap();
    }

    println!("DONE!");
}
```
