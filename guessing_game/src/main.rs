use std::io;
use rand::Rng;

fn main() {
    println!("Welcome to Umesh's Guessing Game made from Rust");
    println!("Just guess the number between 1 and 100 in the lowest times possible!");

    main_game();
    exit_procedure();
}

fn main_game() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    //println!("The secret number is: {}", secret_number);
    let mut tries = 10;
    let mut attempts = 0;

    println!("Guess the number:");

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("An error occurred at the guess input");
        let guess: u32 = guess.trim().parse().expect("Enter a valid integer!");
        attempts += 1;

        if guess != secret_number {

            println!("Try Again! You got {} tries!", tries);
            println!("Number of attempts: {}", attempts);
            if guess > secret_number {
                if guess > 100 {
                    println!("Way too high,try below 100!");
                }
                else {
                    println!("Too high man! Try a lower one below {}:", guess);
                    tries -= 1;
                }
            }

            if guess < secret_number {
                println!("Too low man! Try a higher one above {}.", guess);
                tries -= 1;
            }

            if tries < 4 {
                if guess > secret_number {
                    println!("Too high mate! Try a lower one: ");
                    println!("Stay on alert, you got {} tries", tries);
                }
                else if guess < secret_number {
                    println!("Too low man! Try a Higher one.");
                    println!("Stay on alert, you got {} tries", tries);
                }
            }
        }
        if guess == secret_number {
            println!("Yay! You got it in {} attempts!", attempts);
            break
        }
        if tries == 0 {
            println!("Oh no! You couldn't do it.");
            println!("Better luck next time");
            break;
        }
    }
}

fn exit_procedure() {
    let mut exit_keypress = String::new();

    println!("Thanks for playing");
    println!("Hit enter to exit!");
    // this is going to let the user see the last output before he/she can terminate it willingly

    io::stdin().read_line(&mut exit_keypress).expect("Don't worry, it doesn't matter anyway!");
}
