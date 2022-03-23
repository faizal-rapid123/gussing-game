//including standard IO module for use input capture
use std::io;

mod giverand; // importing random number generator from other module

fn main() //program start
{
    println!("Welcome to the guessing game");
    

    println!("enter a number between 0 and 10");

    
    let mut guess = String::new(); //guess should be mutable beacuse we don't know it's value at compile time

    io::stdin().read_line(&mut guess).expect(" can not read"); //passing refernce of guess inorder to keep ownership
    let guess:i32 = guess.trim().parse().expect("can not parse ");


    if guess > 10 || guess < 0
    {
    panic!("number should be less than and 10"); // aborting the program if the number entered is greater than 10
    }

    let n = giverand::generaterandom();

    if n == guess
    {
        println!("congratulations you have guesssed the correct number")
    }
    else 
    {
        println!("better luck next time, you entered : {} but the number was: {}",guess,n);
    }
    


}
