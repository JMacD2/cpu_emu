pub(crate) mod alu{
    use crate::adders::adders::AddSub64bit;
    use crate::assembler::assembler::InstrType;
    use crate::multiplier::multiplier::Multiplier;
    use crate::bitwise_operator::bitwise_operator::BitwiseOperator;
    use crate::converter::converter::Converter;

    pub(crate) struct Alu {
        // Arithmetic Logic Unit, for Addition, Subtraction, Multiplication, and Bitwise operations
        pub(crate) adder_subtractor_64bit: AddSub64bit,
        pub(crate) multiplier: Multiplier,
        pub(crate) bitwise_operator: BitwiseOperator,
        pub(crate) z: bool, // Zero Flag
        pub(crate) n: bool, // Negative Flag
        pub(crate) o: bool  // Overflow
    }
    impl Alu {

        fn set_flags(&mut self, bits : [bool; 64], carry_out : bool) -> ([bool; 64], bool){
            // Set accumulator flags, for use by branches
            if Converter::bin_to_dec_2s_comp(bits.to_vec()) == 0{
                self.z = true;
                self.n = false;
            }
            else if bits[bits.len()-1]{ // Final bit being set indicates negative value, through two's complement
                self.z = false;
                self.n = true;
            }
            else{
                self.z = false;
                self.n = false;
            }

            self.o = carry_out; // Overflow on a carry-out
            (bits, carry_out)
        }
        pub fn add(&mut self, val0 : [bool; 64], val1 : [bool; 64], incr : bool) -> ([bool; 64], bool){
            // Use Adder to add two numbers
            if incr{
                self.adder_subtractor_64bit.value(val0, val1, true) // DO NOT overwrite flags for a program counter increment
            }
            else{
                let (val, c_out) = self.adder_subtractor_64bit.value(val0, val1, true);
                self.set_flags(val, c_out)
            }
        }

        pub fn sub(&mut self, val0 : [bool; 64], val1 : [bool; 64]) -> ([bool; 64], bool){
            let (val, c_out) = self.adder_subtractor_64bit.value(val0, val1, false);
            self.set_flags(val, c_out)
        }

        pub fn mult(&mut self, val0 : [bool; 64], val1 : [bool; 64]) -> [bool; 64]{
            let mult_val = self.multiplier.value(val0, val1);
            self.set_flags(mult_val, false).0
        }

        pub fn bitwise(&mut self, val0 : [bool; 64], val1 : [bool; 64], instr_type: [bool; 4]) -> [bool; 64]{
            let bitwise_val = self.bitwise_operator.value(val0, val1, instr_type);
            self.set_flags(bitwise_val, false).0
        }
    }

    impl Default for Alu {
        fn default() -> Self {
            Alu{
                adder_subtractor_64bit: AddSub64bit::default(),
                multiplier: Multiplier::default(),
                bitwise_operator: Default::default(),
                z: false, n: false, o: false
            }
        }
    }
}