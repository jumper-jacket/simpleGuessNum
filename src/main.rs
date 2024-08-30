use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    let mut remaining_attempts = 7;
    let mut min_num = 0;
    let mut max_num = 100;

    loop {
        println!("Please input your guess.");
        println!("Remaining attempts {}",remaining_attempts);
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}",guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("too Small!");
                remaining_attempts-=1;
                min_num = guess;
                println!("{} < secret_number < {}\n", min_num, max_num);
            },
            Ordering::Greater =>{
                println!("too Big!\n");
                remaining_attempts-=1;
                max_num = guess;
                println!("{} < secret_number < {}\n", min_num, max_num);
            }, 
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        if remaining_attempts == 0 {
            println!("Game over");
            println!("the secret_number is {}",secret_number);
            break;
        }
    }
}
