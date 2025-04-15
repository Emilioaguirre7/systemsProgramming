use std::{thread, time::Duration};

fn task1() {
    // Task 1
    let operation = |a: i32, b: i32| a * b;
    println!("Task 1 → Result: {}", operation(10, 5));
}

fn task2() {
    // Task 2
    let mut tracker = 0;
    let mut update = || {
        tracker += 1;
        println!("Task 2 → Counter: {}", tracker);
    };

    update();
    update();
}

fn task3() {
    // Task 3

    fn process_vector_map<F>(vec: Vec<i32>, f: F) -> Vec<i32>
    where
        F: Fn(i32) -> i32,
    {
        vec.into_iter().map(f).collect()
    }

    fn process_vector_loop<F>(vec: Vec<i32>, f: F) -> Vec<i32>
    where
        F: Fn(i32) -> i32,
    {
        let mut result = Vec::new();
        for x in vec {
            result.push(f(x));
        }
        result
    }

    let numbers = vec![1, 2, 3];

    let doubled = process_vector_map(numbers.clone(), |x| x * 2);
    let replaced = process_vector_loop(numbers, |x| if x > 2 { 0 } else { x });

    println!("Task 3 → Doubled: {:?}", doubled);
    println!("Task 3 → Replaced: {:?}", replaced);
}

fn task5() {
    // Task 5

    struct ComputeCache<T>
    where
        T: Fn() -> String,
    {
        computation: T,
        value: Option<String>,
    }

    impl<T> ComputeCache<T>
    where
        T: Fn() -> String,
    {
        fn new(computation: T) -> Self {
            ComputeCache {
                computation,
                value: None,
            }
        }

        fn get_result(&mut self) -> String {
            match &self.value {
                Some(v) => {
                    println!("Retrieved from cache instantly!");
                    v.clone()
                }
                None => {
                    println!("Computing (this will take 2 seconds)...");
                    thread::sleep(Duration::from_secs(2));
                    let v = (self.computation)();
                    self.value = Some(v.clone());
                    v
                }
            }
        }
    }

    let mut cache = ComputeCache::new(|| {
        println!("Running expensive operation...");
        thread::sleep(Duration::from_secs(2));
        "Hello, world!".to_string()
    });

    println!("Task 5 → First call:");
    println!("Result: {}", cache.get_result());

    println!("\nTask 5 → Second call:");
    println!("Result (cached): {}", cache.get_result());
}

fn main() {
    // Uncomment the task you want to run
    // task1();
    // task2();
    // task3();
     //task5();
}
