//implement binary value per digit with a size of NUM_OF_SEGMENTS from each individual display
//decoder will use displays not the other way around
//this is a decoder for terminal prinouts, not practical use


pub(crate) struct PrintingDecoder {
    //input from main.rs split into individual numbers to be displayed
    storage: Vec<String>,
}
#[allow(dead_code)]
impl PrintingDecoder {
    pub fn new(string: &str) -> PrintingDecoder {
        PrintingDecoder {
            storage: string.chars().map(|x| x.to_string()).collect(), // |x| is a lambda function
        }
    }

    pub fn stringify_and_add(&mut self) {
        let mut temp_storage: Vec<String> = Vec::new();
        for element in &self.storage {
            match element.as_str() {
                "0" => temp_storage.extend(vec![
                    " _ ".to_string(),
                    "| |".to_string(),
                    "|_|".to_string(),
                ]),
                "1" => temp_storage.extend(vec![
                    "   ".to_string(),
                    "  |".to_string(),
                    "  |".to_string(),
                ]),
                "2" => temp_storage.extend(vec![
                    " _ ".to_string(),
                    " _|".to_string(),
                    "|_ ".to_string(),
                ]),
                "3" => temp_storage.extend(vec![
                    " _ ".to_string(),
                    " _|".to_string(),
                    " _|".to_string(),
                ]),
                "4" => temp_storage.extend(vec![
                    "   ".to_string(),
                    "|_|".to_string(),
                    "  |".to_string(),
                ]),
                "5" => temp_storage.extend(vec![
                    " _ ".to_string(),
                    "|_ ".to_string(),
                    " _|".to_string(),
                ]),
                "6" => temp_storage.extend(vec![
                    "   ".to_string(),
                    "|_ ".to_string(),
                    "|_|".to_string(),
                ]),
                "7" => temp_storage.extend(vec![
                    " _ ".to_string(),
                    "  |".to_string(),
                    "  |".to_string(),
                ]),
                "8" => temp_storage.extend(vec![
                    " _ ".to_string(),
                    "|_|".to_string(),
                    "|_|".to_string(),
                ]),
                "9" => temp_storage.extend(vec![
                    " _ ".to_string(),
                    "|_|".to_string(),
                    "  |".to_string(),
                ]),
                " " => temp_storage.extend(vec![
                    "   ".to_string(),
                    "   ".to_string(),
                    "   ".to_string(),
                ]),
                _ => temp_storage.extend(vec![
                    " _ ".to_string(),
                    "|_ ".to_string(),
                    "|_ ".to_string(),
                ]),
            }
        }
        self.storage = temp_storage;
    }

    pub fn combine(&mut self) -> String {
        let mut result = String::new();
        for skipper in 0..=2 {
            for element in self.storage.iter().skip(skipper).step_by(3) {
                result.push_str(element);
            }
            result.push_str("\n");
        }
        return result;
    }
    pub fn printout(&mut self) {
        self.stringify_and_add();
        print!("{}", self.combine());
    }
}
