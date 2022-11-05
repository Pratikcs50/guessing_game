extern crate rand;
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    
    println!("Guess The Number");
    loop{
        println!("Enter the number");

    let mut guess= String::new();
    io::stdin().read_line(&mut guess).expect("failed to read");

    let secret_number = rand::thread_rng().gen_range(1..
        101);

    let guess: u32 = guess.trim().parse()
    .expect("Please type a number!");

    println!("Your guess {}",guess);

    // println!("The secret number is {}",secret_number);

    match guess.cmp(&secret_number){
        Ordering::Less=> println!("too small"),
        Ordering::Greater=> println!("too big"),
        Ordering::Equal=> {println!("you win");
        break;},
    }

    }
    
   
}


