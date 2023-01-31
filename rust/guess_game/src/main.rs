use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    let mut secrt= rand::thread_rng().gen_range(1,10);
    println!("{}",secrt);

    println!("GUESS GAME \n Guess the number");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).
    expect("Failed to read the input");

    println!("The no. you guessed is :{}", guess);

    let guess:u32=guess.trim().parse().expect("Failed to parse the");



    // Part - 1
    if secrt==guess{
        println!("Dammmn");
    }

    // Part - 2
    // loop{
    match guess.cmp(&secrt){
        Ordering::Less => println!(" Less boy "),
        Ordering::Greater => println!(" Great boy "),
        Ordering::Equal => println!(" Equal Boy Damnnnn YOu win"),
    } 

    // println!("{}",guess.cmp(&secrt));
// }

    // loop{}

}
