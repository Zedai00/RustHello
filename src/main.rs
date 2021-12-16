use std::io;
use std::io::Write;

fn main() {
    print!("Enter Your Name:- ");
    io::stdout().flush().unwrap();
    let mut name = String::new();

    io::stdin().read_line(&mut name).unwrap();
    println!{"Hello {}", name};
}