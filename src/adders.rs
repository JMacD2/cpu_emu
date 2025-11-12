pub(crate) mod adders {
    use crate::logic_gates::logic_gates::{AND, NOT, OR, XOR};

    pub(crate) struct FullAddSub {
        add_and_gate_0: AND,
        add_xor_gate_0: XOR,
        add_and_gate_1: AND,
        add_xor_gate_1: XOR,
        add_or_gate_0: OR,
        sub_not_gate_0: NOT,
        sub_not_gate_1: NOT,
        sub_and_gate_0: AND,
        sub_xor_gate_0: XOR,
        sub_and_gate_1: AND,
        sub_xor_gate_1: XOR,
        sub_or_gate_0: OR
    }
    impl FullAddSub {
        pub fn value(&self, a: bool, b: bool, cin: bool, add: bool) -> (bool, bool) {

            if add {
                let and_0: bool = self.add_and_gate_0.value(a, b);
                let xor_0: bool = self.add_xor_gate_0.value(a, b);
                let and_1: bool = self.add_and_gate_1.value(xor_0, cin);
                let c_out: bool = self.add_or_gate_0.value(and_0, and_1);
                let s: bool = self.add_xor_gate_1.value(xor_0, cin);
                (s, c_out)
            }
            else{
                let a_inv: bool = self.sub_not_gate_0.value(a);
                let and_0: bool = self.sub_and_gate_0.value(a_inv, b);
                let xor_0: bool = self.sub_xor_gate_0.value(a, b);
                let xor_0_inv = self.sub_not_gate_1.value(xor_0);
                let and_1: bool = self.sub_and_gate_1.value(xor_0_inv, cin);
                let br: bool = self.sub_or_gate_0.value(and_0, and_1);
                let d: bool = self.sub_xor_gate_1.value(xor_0, cin);
                (d, br)
            }
        }
    }



    impl Default for FullAddSub {
        fn default() -> Self {
            FullAddSub {
                add_and_gate_0: AND::default(),
                add_xor_gate_0: XOR::default(),
                add_and_gate_1: AND::default(),
                add_xor_gate_1: XOR::default(),
                add_or_gate_0: OR::default(),
                sub_not_gate_0: NOT::default(),
                sub_not_gate_1: NOT::default(),
                sub_and_gate_0: AND::default(),
                sub_xor_gate_0: XOR::default(),
                sub_and_gate_1: AND::default(),
                sub_xor_gate_1: XOR::default(),
                sub_or_gate_0: OR::default(),
            }
        }
    }




    pub(crate) struct AddSub4bit {
        full_add_sub_0 : FullAddSub,
        full_add_sub_1 : FullAddSub,
        full_add_sub_2 : FullAddSub,
        full_add_sub_3 : FullAddSub
    }
    impl AddSub4bit {
        pub fn value(&self, inp0 : [bool; 4], inp1 : [bool; 4], cin : bool, add : bool) -> ([bool; 4], bool) {
            let (val0, c0) = self.full_add_sub_0.value(inp0[0], inp1[0], cin, add);
            let (val1, c1) = self.full_add_sub_1.value(inp0[1], inp1[1], c0, add);
            let (val2, c2) = self.full_add_sub_2.value(inp0[2], inp1[2], c1, add);
            let (val3, c3) = self.full_add_sub_3.value(inp0[3], inp1[3], c2, add);
            ([val0, val1, val2, val3], c3)
        }
    }

    impl Default for AddSub4bit {
        fn default() -> Self {
            AddSub4bit {
                full_add_sub_0: FullAddSub::default(),
                full_add_sub_1: FullAddSub::default(),
                full_add_sub_2: FullAddSub::default(),
                full_add_sub_3: FullAddSub::default(),
            }
        }
    }


    pub struct AddSub16bit {
        add_sub_4bit_0 : AddSub4bit,
        add_sub_4bit_1 : AddSub4bit,
        add_sub_4bit_2 : AddSub4bit,
        add_sub_4bit_3 : AddSub4bit
    }
    impl AddSub16bit {
        pub fn value(&self, inp0 : [bool; 16], inp1 : [bool; 16], cin : bool, add : bool) -> ([bool; 16], bool) {

            let mut nibbles_0 = [[false; 4]; 4];
            let mut nibbles_1 = [[false; 4]; 4];

            nibbles_0[0].copy_from_slice(&inp0[0..4]);
            nibbles_0[1].copy_from_slice(&inp0[4..8]);
            nibbles_0[2].copy_from_slice(&inp0[8..12]);
            nibbles_0[3].copy_from_slice(&inp0[12..16]);

            nibbles_1[0].copy_from_slice(&inp1[0..4]);
            nibbles_1[1].copy_from_slice(&inp1[4..8]);
            nibbles_1[2].copy_from_slice(&inp1[8..12]);
            nibbles_1[3].copy_from_slice(&inp1[12..16]);

            let mut nibbles_return = [[false; 4]; 4];
            let mut carries = [false; 4];

            (nibbles_return[0], carries[0]) = self.add_sub_4bit_0.value(nibbles_0[0], nibbles_1[0], cin, add);
            (nibbles_return[1], carries[1]) = self.add_sub_4bit_1.value(nibbles_0[1], nibbles_1[1], carries[0], add);
            (nibbles_return[2], carries[2]) = self.add_sub_4bit_2.value(nibbles_0[2], nibbles_1[2], carries[1], add);
            (nibbles_return[3], carries[3]) = self.add_sub_4bit_3.value(nibbles_0[3], nibbles_1[3], carries[2], add);

            let mut return16 = [false; 16];

            return16[0..4].copy_from_slice(&nibbles_return[0][0..4]);
            return16[4..8].copy_from_slice(&nibbles_return[1][0..4]);
            return16[8..12].copy_from_slice(&nibbles_return[2][0..4]);
            return16[12..16].copy_from_slice(&nibbles_return[3][0..4]);
            (return16, carries[3])
        }
    }
    impl Default for AddSub16bit {
        fn default() -> Self {
            AddSub16bit {
                add_sub_4bit_0: AddSub4bit::default(),
                add_sub_4bit_1: AddSub4bit::default(),
                add_sub_4bit_2: AddSub4bit::default(),
                add_sub_4bit_3: AddSub4bit::default(),
            }
        }
    }


    pub struct AddSub64bit {
        add_sub_16bit_0 : AddSub16bit,
        add_sub_16bit_1 : AddSub16bit,
        add_sub_16bit_2 : AddSub16bit,
        add_sub_16bit_3 : AddSub16bit
    }
    impl AddSub64bit {
        pub fn value(&self, inp0 : [bool; 64], inp1 : [bool; 64], add : bool) -> ([bool; 64], bool) {

            let mut bit16s_0 = [[false; 16]; 4];
            let mut  bit16s_1 = [[false; 16]; 4];

            bit16s_0[0].copy_from_slice(&inp0[0..16]);
            bit16s_0[1].copy_from_slice(&inp0[16..32]);
            bit16s_0[2].copy_from_slice(&inp0[32..48]);
            bit16s_0[3].copy_from_slice(&inp0[48..64]);

            bit16s_1[0].copy_from_slice(&inp1[0..16]);
            bit16s_1[1].copy_from_slice(&inp1[16..32]);
            bit16s_1[2].copy_from_slice(&inp1[32..48]);
            bit16s_1[3].copy_from_slice(&inp1[48..64]);

            let mut bit16s_return = [[false; 16]; 4];
            let mut carries = [false; 4];

            (bit16s_return[0], carries[0]) = self.add_sub_16bit_0.value(bit16s_0[0], bit16s_1[0], false, add);
            (bit16s_return[1], carries[1]) = self.add_sub_16bit_1.value(bit16s_0[1], bit16s_1[1], carries[0], add);
            (bit16s_return[2], carries[2]) = self.add_sub_16bit_2.value(bit16s_0[2], bit16s_1[2], carries[1], add);
            (bit16s_return[3], carries[3]) = self.add_sub_16bit_3.value(bit16s_0[3], bit16s_1[3], carries[2], add);

            let mut return64 = [false; 64];

            return64[0..16].copy_from_slice(&bit16s_return[0][0..16]);
            return64[16..32].copy_from_slice(&bit16s_return[1][0..16]);
            return64[32..48].copy_from_slice(&bit16s_return[2][0..16]);
            return64[48..64].copy_from_slice(&bit16s_return[3][0..16]);
            (return64, carries[3])
        }
    }
    impl Default for AddSub64bit {
        fn default() -> Self {
            AddSub64bit {
                add_sub_16bit_0: AddSub16bit::default(),
                add_sub_16bit_1: AddSub16bit::default(),
                add_sub_16bit_2: AddSub16bit::default(),
                add_sub_16bit_3: AddSub16bit::default()
            }
        }
    }

}