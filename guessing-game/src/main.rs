use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Enter your guess: ");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read input");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number");
                continue
            },
        };

        match guess.cmp(&secret){
            Ordering::Less => println!("Number is smaller"),
            Ordering::Greater => println!("Number is larger"),
            Ordering::Equal => {
                println!("You guessed correct!");
                break;
            }
        }
    }

}
