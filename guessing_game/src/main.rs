fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    let secret_number = 7; 
    let mut guess = 0; 
    let mut attempts = 0;

    loop {
        guess = 5 + attempts; 
        attempts += 1;

        let result = check_guess(guess, secret_number);

        if result == 0 {
            println!("{} is correct! You guessed it in {} attempts.", guess, attempts);
            break;
        } else if result == 1 {
            println!("{} is too high!", guess);
        } else {
            println!("{} is too low!", guess);
        }
    }
}
