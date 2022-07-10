pub fn run(){
    guessing_game();
}

use std::io::{self, Write};
extern crate rand;
use self::rand::{thread_rng, Rng};
use std::cmp::Ordering::{Less, Equal, Greater};

fn guessing_game(){
    let maxc_k:u8 = 7; 
    println!("Starting the guessing game");
    println!("Guess the number correctly in less than {} tries to win", maxc_k);
    let secret:u8 = thread_rng().gen_range(1..128);
    let mut counter:u8  = 0;

    let mut input = String::new();
    let mut guess:u8;

    loop  { // game loop
        loop {  // loop until you get the next valid guess
            print!("Enter your guess between 1 and {} : ",127);
            io::stdout().flush().expect("Unable to flush stdout");
            input.clear();
            match io::stdin().read_line(&mut input) {
                Ok(_) => (),
                Err(e) => println!("Could not read stdin!  {}", e),
            }
            match input.trim().parse::<u8>() {
                Ok(s) => { 
                    guess = s; 
                    break;
                }
                Err(e) => {
                    println!("Your entry is not a number between 1..{}.  {}",127, e);
                }
            }
        }
        counter += 1;
        println!("You guessed {} in your {}.th try", guess, counter);
        match guess.cmp(&secret){
            Less => { println!("Your guess is low");}
            Equal =>{
                println!("Congrats you win, you guessed {} and secret number is {}", guess, secret);
                break;
            }
            Greater => {
                println!("Your guess is high");
            }
        }
        match counter.cmp(&maxc_k){
            Less => { }
            Equal | Greater => {
                println!("You lost with {} tries", counter);
                break;
            }
        }
    }
    println!();
    println!("You guessed {}, the secret is {}", guess, secret);
}