pub(crate) mod bitwise_operator{
    use num_traits::FromPrimitive;
    use crate::logic_gates::logic_gates::{AND, OR, XOR, NOT};
    use crate::assembler::assembler::InstrType;
    use crate::converter::converter::Converter;
    use crate::vec_to_str;

    pub(crate) struct BitwiseOperator{
        and_operator: [AND; 64], // Use logic gates for bitwise operations
        or_operator: [OR; 64],
        xor_operator: [XOR; 64],
        not_operator: [NOT; 64]
    }
    impl BitwiseOperator{

        // Perform simple bitwise operations

        pub fn value(&self, val0: [bool; 64], val1: [bool; 64], instr_bits: [bool; 4]) -> [bool; 64] {
            let instr_type = FromPrimitive::from_u64(Converter::bin_to_dec_pos_only(instr_bits.to_vec())).unwrap();
            let mut return_bits = [false; 64];
            match instr_type {
                InstrType::AND => {
                    for i in 0..64 {
                        return_bits[i] = self.and_operator[i].value(val0[i], val1[i]); // Iterate over logic gates
                    }
                    return_bits
                },
                InstrType::OR => {
                    for i in 0..64usize {
                        return_bits[i] = self.or_operator[i].value(val0[i], val1[i]);
                    }
                    return_bits
                },
                InstrType::XOR => {
                    for i in 0..64usize {
                        return_bits[i] = self.xor_operator[i].value(val0[i], val1[i]);
                    }
                    return_bits
                },
                InstrType::NOT => {
                    for i in 0..64usize {
                        return_bits[i] = self.not_operator[i].value(val0[i]);
                    }
                    return_bits
                },
                _ => [false; 64]
            }
        }
    }
    impl Default for BitwiseOperator{
        fn default() -> Self {
            BitwiseOperator{
                and_operator: [AND::default(); 64],
                or_operator: [OR::default(); 64],
                xor_operator: [XOR::default(); 64],
                not_operator: [NOT::default(); 64]
            }
        }
    }
}