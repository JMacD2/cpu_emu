pub(crate) mod reg_bank{
    use crate::converter::converter::Converter;
    use crate::reg64::reg64::Reg64;

    pub(crate) struct RegBank {
        // Stores all 15 available registers, and allows for access to them
        pub(crate) registers: [Reg64; 15]
    }
    impl RegBank {
        pub fn set_data(&mut self, index : [bool; 4], data : [bool; 64]){
            let index_dec = Converter::bin_to_dec_pos_only(index.to_vec());
            if index_dec > 0 && index_dec < 15{
                self.registers[index_dec as usize].set_data(data);
            }
        }

        pub fn get_data(&mut self, index : [bool; 4]) -> [bool; 64] {
            let index_dec = Converter::bin_to_dec_pos_only(index.to_vec());
            if index_dec < 15{
                return self.registers[index_dec as usize].get_data();
            }
            [false; 64]
        }
    }

    impl Default for RegBank {
        fn default() -> Self {
            RegBank { registers: [Reg64::default(); 15] }
        }
    }
}