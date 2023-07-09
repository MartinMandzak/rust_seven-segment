mod display;
mod decoder;

use display::Display;
use display::SevenSegmented;
use decoder::PrintingDecoder;

fn main() {
    let mut x = SevenSegmented::new();

    x.set_bit("8");

    println!("saved:");
    println!(" {:07b}", x.get_bit());

    let mut dec = PrintingDecoder::new("123456e");

    println!("{:?}", dec.printout());

}
