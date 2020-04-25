//! generates an answer to your yes/no question,
//! giving one of the possible twenty outcomes.
//! no user input is needed.
//! 
//! TO DO: infinite loop, with user input after each outcome generation to exit the program

use rand::Rng;

mod magicball;
use crate::magicball::answers::answers;

fn main() {

    let texts = answers();

    let choice = rand::thread_rng().gen_range(0,20);

    let outcome = texts[choice as usize];

    println!("{}", outcome);
}
