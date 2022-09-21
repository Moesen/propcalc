extern crate inquire;
mod intro;
mod interface;
use inquire::{InquireError, Select};


fn main() {
    let options = ;
    let ans: Result<&str, InquireError> =
        Select::new("What number?", options).prompt();
    match ans {
        Ok(c) => println!("{}! That was mine too!", c),
        Err(_) => println!("An error occured"),
    };
}
