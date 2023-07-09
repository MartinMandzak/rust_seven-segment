
// https://www.circuitgeeks.com/wp-content/uploads/2021/03/Segments-of-7-Segment-Display.jpg?x24574
pub(crate) enum DictBinary{
    ZERO = 0b1111110,
    ONE = 0b0110000,
    TWO = 0b1101101,
    THREE = 0b1111001,
    FOUR = 0b0110011,
    FIVE = 0b1011011,
    SIX = 0b1011111,
    SEVEN = 0b1110000,
    EIGHT = 0b1111111,
    NINE = 0b1110011,
    SPACE = 0b0000000,
    ERROR = 0b1001111,
}

pub(crate) trait Display {
    fn set_bit(&mut self, value: &str);
    
}

pub(crate) struct SevenSegmented {
    stored_bit: u8,
}

impl SevenSegmented{
    pub fn new() -> SevenSegmented {
        SevenSegmented {
            stored_bit: 0b0000000,
        }
    }
    pub fn get_bit(&self) -> &u8 {
        return &self.stored_bit;
    }

    //always use a vector unless you can prove that a map is faster (c++ creator quote)
    //unused now, will be with a gui, delete the _ prefix
    pub fn _deletelater_get_booleans(&self) -> Vec<bool>{
        let mut result:Vec<bool> = vec![false;7];
        for(i,ch) in format!("{:07b}", self.stored_bit).chars().enumerate(){
            if ch == '1' {
                result[i] = true;
            }
        }
        return result;
    }
}

impl Display for SevenSegmented {
    
    fn set_bit(&mut self, value: &str){
        //transfer string into binary through enum
        match value{
            " " | ""=> self.stored_bit = DictBinary::SPACE as u8,
            "0" => self.stored_bit = DictBinary::ZERO as u8,
            "1" => self.stored_bit = DictBinary::ONE as u8,
            "2" => self.stored_bit = DictBinary::TWO as u8,
            "3" => self.stored_bit = DictBinary::THREE as u8,
            "4" => self.stored_bit = DictBinary::FOUR as u8,
            "5" => self.stored_bit = DictBinary::FIVE as u8,
            "6" => self.stored_bit = DictBinary::SIX as u8,
            "7" => self.stored_bit = DictBinary::SEVEN as u8,
            "8" => self.stored_bit = DictBinary::EIGHT as u8,
            "9" => self.stored_bit = DictBinary::NINE as u8,
            _ => self.stored_bit = DictBinary::ERROR as u8
        }
    }
}
