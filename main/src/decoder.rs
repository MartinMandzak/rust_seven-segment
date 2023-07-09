//implement binary value per digit with a size of NUM_OF_SEGMENTS from each individual display
//decoder will use displays not the other way around
//this is a decoder for terminal prinouts, not practical use 


use crate::display::Display;
use crate::display::DictBinary;



pub(crate) trait Decoder {
    fn combine(&mut self, string: &str, another_string: &str) -> &str;
}

pub(crate) struct PrintingDecoder{
    //input from main.rs split into individual numbers to be displayed
    storage: vec![str],
}

impl PrintingDecoder{
    pub fn new(string: &str) -> PrintingDecoder {
        PrintingDecoder {
            storage: string.split("").collect(),
        }
    }
    //dont forget about the static shit, you dont know enough about it
    pub fn stringify(display: impl Display, database:DictBinary) -> &'static str{
        let dummy_value = "heya";
        return &dummy_value;
    }
    pub fn printout(&mut self) -> &str{
        return self.storage.join();
    }
}

impl Decoder for PrintingDecoder {
    
    fn combine(&mut self, string: &str, another_string: &str) -> &str{
        let dummy_value = "heya";
        return &dummy_value;
    }
    
}