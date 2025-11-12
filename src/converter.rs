pub(crate) mod converter{
    use crate::adders::adders::AddSub64bit;

    pub(crate) struct Converter { // Simple file containing conversions between different data representations
    }

    impl Converter {
        pub(crate) fn bin_to_dec_pos_only(mut bits: Vec<bool>) -> u64 {
            let mut total: u128 = 0;
            for i in 0..bits.len() {
                if bits[i] {
                    total += 2_u128.pow(i as u32);
                }
            }
            total as u64
        }

        pub(crate) fn bin_to_dec_2s_comp(mut bits: Vec<bool>) -> i64 {
            let mut total: i128 = 0;
            for i in 0..bits.len() {
                if bits[i] {
                    if i == bits.len() - 1 {
                        total += -1 * 2u128.pow(i as u32) as i128
                    }
                    else {
                        total += 2u128.pow(i as u32) as i128
                    }
                }
            }
            total as i64
        }

        pub fn bin_to_hex(bits: Vec<bool>) -> String {
            let mut output: String = String::from("");
            let fours: usize = ((bits.len() as f64 / 4.0).floor() as usize) - 1;
            for i in 0..fours {
                output = Self::dec_to_hex(Self::bin_to_dec_pos_only([bits[i*4], bits[i*4+1], bits[i*4+2], bits[i*4+3]].to_vec()) as u32) + output.as_str();
            }
            output
        }

        pub fn bin_flip_sign(mut val:[bool; 64]) -> [bool; 64] {
            let mut one_bit : [bool; 64] = [false; 64];
            one_bit[0] = true;
            let add_sub : AddSub64bit = AddSub64bit::default();
            if val[val.len()-1] {
                val = add_sub.value(val, one_bit, false).0;
                for i in 0..val.len(){
                    val[i] = !val[i];
                }
                val
            }
            else{
                for i in 0..val.len(){
                    val[i] = !val[i];
                }
                add_sub.value(val, one_bit, true).0
            }
        }

        pub fn dec_to_bin_pos_only(mut val: u64, size: u8) -> Vec<bool> {
            let mut bits_vec: Vec<bool> = Vec::new();
            while val > 0 {
                bits_vec.push(val % 2 != 0);
                val = (val as f64 / 2.0).floor() as u64;
            }
            Self::set_size(bits_vec, size)
        }

        pub fn dec_to_bin_2s_comp(mut val: i64) -> [bool; 64] {
            if val < 0{
                val *= -1;
                Self::bin_flip_sign(Self::dec_to_bin_pos_only(val as u64, 64).try_into().unwrap())
            }
            else{
                Self::dec_to_bin_pos_only(val as u64, 64).try_into().unwrap()
            }
        }

        pub fn set_size(mut bits: Vec<bool>, size: u8) -> Vec<bool> {
            if size < 1 {
                return Vec::new();
            }
            // Extend OR truncate
            while bits.len() < size as usize {
                bits.push(false);
            }
            if bits.len() > size as usize{
                bits.truncate(size as usize);
            }
            bits
        }

        pub fn bit48_to64(data: [bool; 48]) -> [bool; 64] {
            let mut return_bits = [false; 64];
            return_bits[0..48].copy_from_slice(&data);
            return_bits
        }

        pub fn dec_to_hex(mut dec: u32) -> String {
            let mut output: String = String::from("");
            while dec != 0 {
                let remainder: u32 = dec % 16;
                if remainder < 10 {
                    output = String::from(char::from_u32(remainder + 48).unwrap()) + &output;
                } else {
                    output = String::from(char::from_u32(remainder + 55).unwrap()) + &output;
                }
                dec /= 16;
            }
            output
        }

        pub fn hex_resize(hex: String) -> String {
            // Resizes input hex values to a 16-character 64-bit hexadecimal
            // Reverses hex string to ease input into datatypes
            let mut reversed = String::from("");
            let mut split = hex.split("");
            for i in 0..hex.len() { reversed += split.nth(hex.len() - i - 1).unwrap(); }
            while reversed.len() < 16 { reversed += "0"; }
            reversed
        }

        pub fn hex_char_to_dec(character : char) -> u32{
            let int_val = character as u32;
            if int_val >= 48 && int_val <= 57{
                return int_val - 48;
            }
            else if int_val >= 65 && int_val <= 70{
                return int_val - 55;
            }
            0
        }

        pub fn hex_val_to_dec(str : String) -> u32{
            let mut return_val: u32 = 0;
            for i in 0..str.len(){
                return_val += Self::hex_char_to_dec(str.chars().nth(str.len()-i-1).unwrap()) * 16^(i as u32);
            }
            return_val
        }

        fn hex_char_to_bin(character : char) -> [bool; 4]{
            match character {
                '1' => {
                    [true, false, false, false]
                }
                '2' => {
                    [false, true, false, false]
                }
                '3' => {
                    [true, true, false, false]
                }
                '4' => {
                    [false, false, true, false]
                }
                '5' => {
                    [true, false, true, false]
                }
                '6' => {
                    [false, true, true, false]
                }
                '7' => {
                    [true, true, true, false]
                }
                '8' => {
                    [false, false, false, true]
                }
                '9' => {
                    [true, false, false, true]
                }
                'A' => {
                    [false, true, false, true]
                }
                'B' => {
                    [true, true, false, true]
                }
                'C' => {
                    [false, false, true, true]
                }
                'D' => {
                    [true, false, true, true]
                }
                'E' => {
                    [false, true, true, true]
                }
                'F' => {
                    [true, true, true, true]
                }
                _ => {
                    [false, false, false, false]
                }
            }
        }


        pub fn hex_val_to_bin(mut hex : String) -> Vec<bool> {
            let mut binary_vec: Vec<bool> = Vec::new();
            for character in hex.chars().rev(){
                let char_bits = Self::hex_char_to_bin(character);
                for i in 0..4{
                    binary_vec.push(char_bits[i]);
                }
            }
            binary_vec
        }
    }
}