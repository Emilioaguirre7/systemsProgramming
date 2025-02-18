fn sum(total: &mut i32, low: i32, high: i32) {
    *total = 0; 
    for i in low..=high {
        *total += i;
    }
}

fn main() {
    let mut total = 0; 
    let low = 0;
    let high = 100;

    sum(&mut total, low, high);

    println!("Total sum from {} to {}: {}", low, high, total); 
}
