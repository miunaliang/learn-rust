use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("guess the number");
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("secret number is {}", secret_number);
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("fail");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("you guess: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }
}
