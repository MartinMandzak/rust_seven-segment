//implement binary value per digit with a size of NUM_OF_SEGMENTS from each individual display
//decoder will use displays not the other way around
//this is a decoder for terminal prinouts, not practical use 


use crate::display::SevenSegmented;

pub(crate) struct PrintingDecoder{
    //input from main.rs split into individual numbers to be displayed
    storage: Vec<String>,
}

impl PrintingDecoder{
    pub fn new(string: &str) -> PrintingDecoder {
        PrintingDecoder {
            storage: string.chars().map(|x| x.to_string()).collect(), // |x| is a lambda function
        }
    }
    //dont forget about the static shit, you dont know enough about it
    pub fn stringify_and_add_display(&mut self, display: SevenSegmented){
        match display.get_bit(){
            0b1111110 => self.storage.extend(vec![" _ ".to_string(),"| |".to_string(),"|_|".to_string()]),
            0b0110000 =>self.storage.extend(vec!["   ".to_string(),"  |".to_string(),"  |".to_string()]),
            0b1101101 =>self.storage.extend(vec![" _ ".to_string()," _|".to_string(),"|_ ".to_string()]),
            0b1111001 =>self.storage.extend(vec![" _ ".to_string()," _|".to_string()," _|".to_string()]),
            0b0110011 =>self.storage.extend(vec!["   ".to_string(),"|_|".to_string(),"  |".to_string()]),
            0b1011011 =>self.storage.extend(vec![" _ ".to_string(),"|_ ".to_string()," _|".to_string()]),
            0b1011111 =>self.storage.extend(vec!["   ".to_string(),"|_ ".to_string(),"|_|".to_string()]),
            0b1110000 =>self.storage.extend(vec![" _ ".to_string(),"  |".to_string(),"  |".to_string()]),
            0b1111111 =>self.storage.extend(vec![" _ ".to_string(),"|_|".to_string(),"|_|".to_string()]),
            0b1110011 =>self.storage.extend(vec![" _ ".to_string(),"|_|".to_string(),"  |".to_string()]),
            0b0000000 =>self.storage.extend(vec!["   ".to_string(),"   ".to_string(),"   ".to_string()]),
            _ =>self.storage.extend(vec![" _ ".to_string(),"|_ ".to_string(),"|_ ".to_string()])
        }
    }

    pub fn stringify_and_add(&mut self){
        let mut temp_storage: Vec<String> = Vec::new();
        for element in &self.storage{
            match element.as_str(){
                "0" => temp_storage.extend(vec![" _ ".to_string(),"| |".to_string(),"|_|".to_string()]),
                "1" =>temp_storage.extend(vec!["   ".to_string(),"  |".to_string(),"  |".to_string()]),
                "2" =>temp_storage.extend(vec![" _ ".to_string()," _|".to_string(),"|_ ".to_string()]),
                "3" =>temp_storage.extend(vec![" _ ".to_string()," _|".to_string()," _|".to_string()]),
                "4" =>temp_storage.extend(vec!["   ".to_string(),"|_|".to_string(),"  |".to_string()]),
                "5" =>temp_storage.extend(vec![" _ ".to_string(),"|_ ".to_string()," _|".to_string()]),
                "6" =>temp_storage.extend(vec!["   ".to_string(),"|_ ".to_string(),"|_|".to_string()]),
                "7" => temp_storage.extend(vec![" _ ".to_string(),"  |".to_string(),"  |".to_string()]),
                "8" =>temp_storage.extend(vec![" _ ".to_string(),"|_|".to_string(),"|_|".to_string()]),
                "9" =>temp_storage.extend(vec![" _ ".to_string(),"|_|".to_string(),"  |".to_string()]),
                " " =>temp_storage.extend(vec!["   ".to_string(),"   ".to_string(),"   ".to_string()]),
                _ =>temp_storage.extend(vec![" _ ".to_string(),"|_ ".to_string(),"|_ ".to_string()])
            }
        }
        self.storage = temp_storage;
    }

    pub fn combine() -> &'static str{
        return "heya";
    }
    pub fn printout(&mut self) -> &str{
        return "heya";
    }
}
