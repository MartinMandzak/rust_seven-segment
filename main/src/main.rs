mod decoder;

use decoder::PrintingDecoder;
use std::io;

fn main() {
    println!("Please enter your number: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let mut x = PrintingDecoder::new(&input.trim_end());

    x.printout();
}
