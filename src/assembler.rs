pub(crate) mod assembler{

    use num_derive::FromPrimitive;
    use crate::converter::converter::Converter;

    #[repr(u8)]
    #[derive(Clone, FromPrimitive)]
    pub(crate) enum InstrType {
        // Instruction Type enumeration
        OTH = 0,
        ADD = 1,
        SUB = 2,
        B = 3,
        LDR = 4,
        STR = 5,
        HLT = 6,
        OUT = 7,
        MULT = 8,
        CMP = 9,
        AND = 10,
        OR = 11,
        XOR = 12,
        NOT = 13,
        FLIP = 14
    }

    #[repr(u8)]
    #[derive(Clone, FromPrimitive)]
    pub(crate) enum BranchConditions {
        // Branch Type enumeration
        B = 0,
        BEQ = 1,
        BNE = 2,
        BLT = 3,
        BGT = 4,
        BLE = 5,
        BGE = 6,
        OTH = 7
    }

    #[derive(Clone)]
    pub(crate) struct ParsedInstruction {
        // Parsed Instruction Object
        pub(crate) instr_type: InstrType,
        pub(crate) branch_condition : BranchConditions,
        pub(crate) addr : [bool; 48],
        pub(crate) return_register : [bool; 4],
        pub(crate) reg_0 : bool,
        pub(crate) reg_1 : bool,
        pub(crate) ascii : bool,
        pub(crate) input_val_0 : [bool; 16],
        pub(crate) input_val_1 : [bool; 16]
    }
    impl ParsedInstruction {
        pub fn clear(&mut self){
            self.instr_type = InstrType::OTH;
            self.branch_condition = BranchConditions::OTH;
            self.addr = [false; 48];
            self.return_register = [false; 4];
            self.reg_0 = false;
            self.reg_1 = false;
            self.ascii = false;
            self.input_val_0 = [false; 16];
            self.input_val_1 = [false; 16];
        }
    }

    impl Default for ParsedInstruction {
        fn default() -> Self {
            ParsedInstruction {
                instr_type : InstrType::OTH,
                branch_condition : BranchConditions::OTH,
                addr : [false; 48], return_register : [false; 4],
                reg_0 : false, reg_1 : false, ascii : false,
                input_val_0 : [false; 16], input_val_1 : [false; 16]
            }
        }
    }

    pub(crate) struct Assembler {} // Convert a line from the instruction input file first into an intermediate representation (ParsedInstruction), then into binary

    impl Assembler {

        pub fn get_type(&self, line : String) -> InstrType { // Get instruction type
            match line.split(" ").nth(0).unwrap() {
                "ADD" => InstrType::ADD,
                "SUB" => InstrType::SUB,
                "MULT" => InstrType::MULT,
                "LDR" => InstrType::LDR,
                "STR" => InstrType::STR,
                "HLT" => InstrType::HLT,
                "OUT" => InstrType::OUT,
                "CMP" => InstrType::CMP,
                "AND" => InstrType::AND,
                "OR" => InstrType::OR,
                "XOR" => InstrType::XOR,
                "NOT" => InstrType::NOT,
                "FLIP" => InstrType::FLIP,
                _ => {
                    if line.split(" ").nth(0).unwrap().chars().nth(0).unwrap() == 'B' {
                        return InstrType::B;
                    }
                    InstrType::OTH
                }
            }
        }

        pub fn get_condition(&self, line : String) -> BranchConditions { // Derive branch condition
            match line.split(" ").nth(0).unwrap() {
                "B" => BranchConditions::B,
                "BEQ" => BranchConditions::BEQ,
                "BNE" => BranchConditions::BNE,
                "BLT" => BranchConditions::BLT,
                "BGT" => BranchConditions::BGT,
                "BLE" => BranchConditions::BLE,
                "BGE" => BranchConditions::BGE,
                _ => BranchConditions::OTH
            }
        }

        pub fn parse_line_data(&mut self, line : String) -> ParsedInstruction {

            // Convert line into an intermediate reperesentation

            let mut parsed_instr : ParsedInstruction = ParsedInstruction::default();
            let line_cpy = line.clone();
            let type_val = self.get_type(line.clone());
            parsed_instr.instr_type = self.get_type(line);

            match type_val {

                InstrType::ADD|InstrType::SUB|InstrType::MULT|InstrType::AND|InstrType::OR|InstrType::XOR => {

                    let mut split = line_cpy.split(" ");

                    let split_0_parse = &split.clone().nth(1).unwrap().to_string()[1..split.clone().nth(1).unwrap().to_string().clone().len()-1];
                    let target_reg_number : u8 = split_0_parse.parse().expect("Failed to parse target register number");
                    parsed_instr.return_register = Converter::dec_to_bin_pos_only(target_reg_number as u64, 4).try_into().unwrap();

                    let split_1_parse = split.clone().nth(2).unwrap().to_string().replace(&['R','#',','][..], "");
                    let literal : i16 = split_1_parse.parse().expect("Failed to parse target register number");
                    parsed_instr.input_val_0 = Converter::dec_to_bin_pos_only(literal as u64, 16).try_into().unwrap();

                    if split.clone().nth(2).unwrap().chars().nth(0).unwrap() != '#'{ parsed_instr.reg_0 = true; }

                    let split_2_parse = split.clone().nth(3).unwrap().to_string().replace(&['R','#',','][..], "");
                    let literal : i16 = split_2_parse.parse().expect("Failed to parse target register number");
                    parsed_instr.input_val_1 = Converter::dec_to_bin_pos_only(literal as u64, 16).try_into().unwrap();

                    if split.nth(3).unwrap().chars().nth(0).unwrap() != '#'{ parsed_instr.reg_1 = true;} // '#' indicates literal, 'R' indicates register reference
                }

                InstrType::NOT|InstrType::FLIP => {
                    let mut split = line_cpy.split(" ");

                    let split_0_parse = &split.clone().nth(1).unwrap().to_string()[1..split.clone().nth(1).unwrap().to_string().clone().len()-1];
                    let target_reg_number : u8 = split_0_parse.parse().expect("Failed to parse target register number");
                    parsed_instr.return_register = Converter::dec_to_bin_pos_only(target_reg_number as u64, 4).try_into().unwrap();

                    let split_1_parse = split.clone().nth(2).unwrap().to_string().replace(&['R','#',','][..], "");
                    let literal : i16 = split_1_parse.parse().expect("Failed to parse target register number");
                    parsed_instr.input_val_0 = Converter::dec_to_bin_pos_only(literal as u64, 16).try_into().unwrap();

                    if split.clone().nth(2).unwrap().chars().nth(0).unwrap() != '#'{ parsed_instr.reg_0 = true; }
                }

                InstrType::CMP => {
                    let mut split = line_cpy.split(" ");

                    let split_0_parse = &split.clone().nth(1).unwrap().to_string()[1..split.clone().nth(1).unwrap().to_string().clone().len()-1];
                    let literal : i16 = split_0_parse.parse().expect("Failed to parse target register number");
                    parsed_instr.input_val_0 = Converter::dec_to_bin_pos_only(literal as u64, 16).try_into().unwrap();

                    if split.clone().nth(1).unwrap().chars().nth(0).unwrap() != '#'{ parsed_instr.reg_0 = true; }

                    let split_1_parse = split.clone().nth(2).unwrap().to_string().replace(&['R','#',','][..], "");
                    let literal : i16 = split_1_parse.parse().expect("Failed to parse target register number");
                    parsed_instr.input_val_1 = Converter::dec_to_bin_pos_only(literal as u64, 16).try_into().unwrap();

                    if split.nth(2).unwrap().chars().nth(0).unwrap() != '#'{ parsed_instr.reg_1 = true;}
                }

                InstrType::STR| InstrType::LDR => {
                    let split1 = line_cpy.split(" ").nth(1).unwrap();
                    let reg_number : i32 = split1[1..(split1.len()-1)].parse().unwrap();
                    let split2 = line_cpy.split(" ").nth(2).unwrap();
                    let hex_str: String = split2[1..split2.len()].parse().unwrap();
                    parsed_instr.addr = Converter::hex_val_to_bin(hex_str).try_into().unwrap();
                    parsed_instr.return_register = Converter::dec_to_bin_pos_only(reg_number as u64, 4).try_into().unwrap();
                }

                InstrType::B => {
                    parsed_instr.branch_condition = self.get_condition(line_cpy.clone());
                    let address_string = line_cpy.split(" ").nth(1).unwrap();
                    let hex_str: String = address_string[1..address_string.len()].parse().unwrap();
                    parsed_instr.addr = Converter::hex_val_to_bin(hex_str).try_into().unwrap();
                }

                InstrType::OUT => {
                    let mut out_split = line_cpy.split(" ");
                    if out_split.clone().nth(1).unwrap().chars().nth(0).unwrap() == 'A'{
                        parsed_instr.ascii = true;
                    }
                    else{
                        parsed_instr.ascii = false;
                    }

                    let reg_str = out_split.nth(2).unwrap();
                    parsed_instr.return_register = Converter::dec_to_bin_pos_only(reg_str[1..reg_str.len()].parse().unwrap(), 4).try_into().unwrap();
                }

                _ => {}

            }
            parsed_instr
        }

        pub fn code_generation(&self, parsed_instruction: ParsedInstruction) -> [bool; 64] { // Convert partial representation into 64-bit binary - described in design document
            let mut return_bits = [false; 64];
            let mut instr_nibble = [false; 4];
            instr_nibble = Converter::dec_to_bin_pos_only(parsed_instruction.instr_type.clone() as u64, 4).try_into().unwrap();

            return_bits[0..4].copy_from_slice(&instr_nibble);
            match parsed_instruction.instr_type {

                InstrType::ADD|InstrType::SUB|InstrType::MULT|InstrType::AND|InstrType::OR|InstrType::XOR => {
                    return_bits[4..8].copy_from_slice(&parsed_instruction.return_register);
                    return_bits[8] = parsed_instruction.reg_0;
                    return_bits[9..25].copy_from_slice(&parsed_instruction.input_val_0);
                    return_bits[25] = parsed_instruction.reg_1;
                    return_bits[26..42].copy_from_slice(&parsed_instruction.input_val_1);
                }

                InstrType::NOT|InstrType::FLIP => {
                    return_bits[4..8].copy_from_slice(&parsed_instruction.return_register);
                    return_bits[8] = parsed_instruction.reg_0;
                    return_bits[9..25].copy_from_slice(&parsed_instruction.input_val_0);
                }

                InstrType::CMP => {
                    return_bits[4] = parsed_instruction.reg_0;
                    return_bits[5..21].copy_from_slice(&parsed_instruction.input_val_0);
                    return_bits[21] = parsed_instruction.reg_1;
                    return_bits[22..38].copy_from_slice(&parsed_instruction.input_val_1);
                }

                InstrType::LDR|InstrType::STR => {
                    return_bits[4..8].copy_from_slice(&parsed_instruction.return_register);
                    return_bits[8..40].copy_from_slice(&parsed_instruction.addr);
                }

                InstrType::B => {
                    let mut condition_nibble = [false; 4];
                    condition_nibble = Converter::dec_to_bin_pos_only(parsed_instruction.branch_condition.clone() as u64, 4).try_into().unwrap();
                    return_bits[4..8].copy_from_slice(&condition_nibble);
                    return_bits[8..56].copy_from_slice(&parsed_instruction.addr);
                }

                InstrType::OUT => {
                    return_bits[4..8].copy_from_slice(&parsed_instruction.return_register);
                    return_bits[8] = parsed_instruction.ascii;
                }

                _ => {}

            }
            return_bits
        }

        pub fn assemble(&mut self, line : String) -> [bool; 64] {
            let parsed_instruction: ParsedInstruction = self.parse_line_data(line); // Generate intermediate representation
            self.code_generation(parsed_instruction) // Convert intermediate representation into binary and return
        }
    }
}