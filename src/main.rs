//including standard IO module for use input capture
use std::io;
use rand::{thread_rng, Rng};

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

    let mut rng = thread_rng();

    // Exclusive range
    let n: u32 = rng.gen_range(0..10);
    let n:i32 = n as i32; //parsing n in i32 as guess is also in i32 formate


    if n == guess
    {
        println!("congratulations you have guesssed the correct number")
    }
    else 
    {
        println!("better luck next time, you entered : {} but the number was: {}",guess,n);
    }
    


}
