fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    let numbers: [i32; 10] = [1,2,3,4,5,6,7,8,9,10];

    for &num in numbers.iter() {
        if num % 3 == 0 && num % 5 == 0 {
            println!("{}: FizzBuzz", num);
        } else if num % 3 == 0 {
            println!("{}: Fizz", num);
        } else if num % 5 == 0 {
            println!("{}: Buzz", num);
        } else {
            let even_or_odd = if is_even(num) { "even" } else { "odd" };
            println!("{} is {}", num, even_or_odd);
        }
    }

    let mut sum = 0;
    let mut i = 0;
    while i < numbers.len() {
        sum += numbers[i];
        i += 1;
    }
    println!("Sum of all numbers: {}", sum);

    let mut max_number = numbers[0];
    for &num in numbers.iter() {
        if num > max_number {
            max_number = num;
        }
    }
    println!("Largest number in the array: {}", max_number);
}
