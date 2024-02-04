use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    println!("Giues the number");

    let secret_number = rand::thread_rng().gen_range(1, 100);

    println!("The secret umber is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess= String::new();
    
        io::stdin()
            .read_line( &mut guess)
            .expect("Faied to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("{}","Too big".red()),
            Ordering::Less => println!("{}","Too smaoll!!".green()),
            Ordering::Equal => {
                println!("{}","You Win!!".blue());
                break;
            },  
        }
    
    }

   
}

