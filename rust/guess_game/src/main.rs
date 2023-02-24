// Captain272 Ramp-Rust 
use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {

    let secrt= rand::thread_rng().gen_range(1,10);
    println!("{}",secrt);
    println!("GUESS GAME \n Guess the number");
    let mut guess_1 = String::new();
    io::stdin().read_line(&mut guess_1).
    expect("Failed to read the input");
    println!("The no. you guessed is :{}", guess_1);

    let guess: u32 = guess_1.trim().parse().expect("Failed to parse the");

    // Part - 1
    println!("{}",guess);
    if secrt==guess{
        println!("Dammmn");
    }
    else{
        println!("Craaamp");
    }

    // Part - 2
    loop{
    
    let mut guess_2 = String::new();
    println!("Enter a number to guess :");
    io::stdin().read_line(&mut guess_2).
    expect("Failed to read the input");
    let guess: u32= match guess_2.trim().parse(){
        Ok(num)=>num,
        Err(er)=>{println!("Failed to parse : {}",er); continue;},
    };

    println!("The no. you guessed is :{}", guess);
    println!("Comparing");
    match guess.cmp(&secrt){
        Ordering::Less =>println!("{}","The number is less".red()),
        Ordering::Greater => println!("{}","The number is greater".red()),
        Ordering::Equal => {println!("{}"," Equal Boy Damnnnn YOu win".green()); break;},
    } 
    


}


}
