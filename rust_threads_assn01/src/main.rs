use std::thread;
use std::time::Duration;

fn main() {
    println!("Main thread starting");
    
    // Create a vector to store thread handles
    let mut handles = vec![];

    // Spawn 3 threads
    for i in 1..=3 {
        // Spawn a thread and store its handle
        let handle = thread::spawn(move || {
            println!("Thread {} starting", i);
            thread::sleep(Duration::from_millis(500));
            println!("Thread {} finished", i);
        });

        // Store the handle
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    println!("All threads completed.");
}
