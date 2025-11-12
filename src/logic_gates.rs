pub(crate) mod logic_gates { // Contains logic gates, built of transistors or of other logic gates
    use num_derive::FromPrimitive;
    use crate::transistors::transistors::Nmos;
    use crate::transistors::transistors::Pmos;


    #[derive(Clone, Copy)]
    pub struct AND { gate_0: Nmos, gate_1: Nmos }
    impl AND {
        pub fn value(&self, inp_0: bool, inp_1: bool) -> bool {
            self.gate_1.value(inp_1, self.gate_0.value(inp_0, true))
        }
    }
    impl Default for AND{ fn default() -> Self { AND{gate_0 : Nmos {}, gate_1 : Nmos {}} } }

    #[derive(Clone, Copy)]
    pub(crate) struct NOT { gate_0: Pmos }
    impl NOT { pub fn value(&self, input: bool) -> bool { self.gate_0.value(input, true) } }
    impl Default for NOT { fn default() -> Self { NOT {gate_0 : Pmos {}} } }

    #[derive(Clone, Copy)]
    pub(crate) struct XOR {
        nor_gate_0: NOR,
        nor_gate_1: NOR,
        nor_gate_2: NOR,
        nor_gate_3: NOR,
        nor_gate_4: NOR
    }

    impl XOR {
        pub fn value(&self, input_0: bool, input_1: bool) -> bool {
            let nor_0_1 = self.nor_gate_0.value(input_0, input_1);
            let nor_0_nor_0_1 = self.nor_gate_1.value(input_0, nor_0_1);
            let nor_nor_0_1_1 = self.nor_gate_2.value(nor_0_1, input_1);
            let nor_nors = self.nor_gate_3.value(nor_0_nor_0_1, nor_nor_0_1_1);
            self.nor_gate_4.value(nor_nors, nor_nors)
        }
    }
    impl Default for XOR {
        fn default() -> Self {
            XOR {
                nor_gate_0: NOR::default(),
                nor_gate_1: NOR::default(),
                nor_gate_2: NOR::default(),
                nor_gate_3: NOR::default(),
                nor_gate_4: NOR::default(),
            }
        }
    }

    #[derive(Clone, Copy)]
    pub(crate) struct OR {
        gate_0: Nmos,
        gate_1: Nmos
    }
    impl OR {
        pub fn value(&self, input_0: bool, input_1: bool) -> bool { self.gate_0.value(input_0, true) || self.gate_1.value(input_1, true) }
    }
    impl Default for OR {
        fn default() -> Self { OR {gate_0 : Nmos {}, gate_1 : Nmos {}} }
    }

    #[derive(Clone, Copy)]
    pub(crate) struct NOR {
        or_gate: OR,
        not_gate: NOT
    }
    impl NOR {
        pub fn value(&self, input_0: bool, input_1: bool) -> bool { self.not_gate.value(self.or_gate.value(input_0, input_1)) }
    }
    impl Default for NOR {
        fn default() -> Self { NOR {or_gate : OR::default(), not_gate : NOT::default()} }
    }
}