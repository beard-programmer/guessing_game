use std::{ io, ops::RangeInclusive };

use rand::Rng;

fn main() {
    const LOWER_LIMIT: u32 = 1;
    const UPPER_LIMIT: u32 = 15;
    const GAMING_RANGE: RangeInclusive<u32> = LOWER_LIMIT..=UPPER_LIMIT;

    println!("Guess the number!");
    let secret_number: u32 = generate_secret(GAMING_RANGE);
    let mut number_of_attempts: u32 = 0;

    loop {
        println!("Please, input ur guess, between {LOWER_LIMIT} and {UPPER_LIMIT}");
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read line");
        let guess: u32 = match user_input.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Input is not a non negative number, please try again!");
                continue;
            }
        };

        if !guess_within_gaming_range(&guess) {
            println!(
                "Given guess is not within gaming range; it violates the rules: {LOWER_LIMIT} <= {guess}(Your guess) <= {UPPER_LIMIT}"
            );
            continue;
        }
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small."),
            std::cmp::Ordering::Equal => {
                println!("Nice, you did it!");
                break;
            }
            std::cmp::Ordering::Greater => println!("Too big."),
        }
        number_of_attempts += 1;
        if needs_a_tip(number_of_attempts) {
            println!("Pss. Secret number is {secret_number}.");
        }
    }

    fn generate_secret(range: RangeInclusive<u32>) -> u32 {
        rand::thread_rng().gen_range(range)
    }

    fn guess_within_gaming_range(guess: &u32) -> bool {
        GAMING_RANGE.contains(&guess)
    }

    fn needs_a_tip(number_of_attempts: u32) -> bool {
        UPPER_LIMIT / number_of_attempts <= 1
    }
}