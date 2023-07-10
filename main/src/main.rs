mod display;
mod decoder;

use std::io;
use decoder::PrintingDecoder;

fn main() {
    println!("Please enter your number: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut x = PrintingDecoder::new(&input);

    x.printout();

}
