use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;

const TERMINATION_SIGNAL: i32 = -1;

fn main() {
    const ITEM_COUNT: usize = 20;

    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));

    let mut handles = vec![];

    // 2 Producers
    for i in 0..2 {
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || producer(i, tx_clone, ITEM_COUNT / 2));
        handles.push(handle);
    }

    // 3 Consumers
    for i in 0..3 {
        let rx_clone = Arc::clone(&rx);
        let handle = thread::spawn(move || consumer(i, rx_clone));
        handles.push(handle);
    }

    // Send termination signals (after all producers finish)
    for _ in 0..3 {
        tx.send(TERMINATION_SIGNAL).unwrap();
    }

    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }

    println!("All items have been produced and consumed!");
}

fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    let mut rng = rand::thread_rng();
    for _ in 0..item_count {
        let value = rng.gen_range(1..101);
        println!("Producer {} produced {}", id, value);
        tx.send(value).unwrap();
        thread::sleep(Duration::from_millis(100));
    }
}

fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    loop {
        let val = rx.lock().unwrap().recv().unwrap();
        if val == TERMINATION_SIGNAL {
            println!("Consumer {} received termination signal", id);
            break;
        } else {
            println!("Consumer {} processed {}", id, val);
        }
    }
}
