pub(crate) mod control_unit{
    use std::fs::File;
    use std::io::Write;
    use num_derive::FromPrimitive;
    use crate::reg64::reg64::Reg64;
    use crate::reg_bank::reg_bank::RegBank;
    use crate::caches::caches::DataAccessManager;
    use crate::assembler::assembler::{BranchConditions, InstrType, ParsedInstruction};
    use crate::converter::converter::Converter;
    use crate::alu::alu::Alu;
    use num_traits::FromPrimitive;
    use crate::vec_to_str;

    #[repr(u8)]
    #[derive(Clone, FromPrimitive)]
    pub(crate) enum CpuState {
        Fetch = 0,
        Decode = 1,
        Execute = 2,
        Stall = 3,
        MemoryComp = 4
    }

    pub(crate) struct ControlUnit {
        pub(crate) alu : Alu,
        pub(crate) memory_instr_reg : Reg64,
        pub(crate) memory_instr_stall : bool,
        pub(crate) memory_data_reg : Reg64,
        pub(crate) memory_data_stall : bool,
        pub(crate) pc: Reg64,
        pub(crate) register_bank : RegBank,
        pub(crate) halt : bool,
        pub(crate) data_access_manager: DataAccessManager,
        pub(crate) state : CpuState,
        pub(crate) decoded_instruction : ParsedInstruction
    }

    impl ControlUnit {

        pub fn tick(&mut self){ // Called by clock

            match self.state{

                CpuState::Fetch => {

                    let mut read_addr = [false; 48];
                    read_addr[0..48].copy_from_slice(&self.pc.get_data()[0..48]);
                    let (data, cache_hit) = self.data_access_manager.read(read_addr); // Read from cache where possible

                    if cache_hit{
                        self.memory_instr_reg.set_data(data);
                        self.state = CpuState::Decode;
                    }
                    else{
                        self.memory_instr_stall = true;
                        self.state = CpuState::Stall // Wait for instruction in lieu of a cache hit
                    }

                    // Increment PC using ALU
                    let mut one_bit : [bool; 64] = [false; 64];
                    one_bit[6] = true;
                    let (increment, carry_out) = self.alu.add(self.pc.get_data(), one_bit, true);
                    if carry_out{ self.pc.set_data([false; 64]); }
                    else{ self.pc.set_data(increment); }

                },

                CpuState::Decode => {

                    // Converts binary representation back into an intermediate representation for ease of use
                    // Binary representation decodings can be found in the design document

                    self.decoded_instruction.clear();
                    let mdr_data = self.memory_instr_reg.get_data();

                    let mut zero : bool = true;
                    for bit in mdr_data{
                        if bit{
                            zero = false;
                        }
                    }

                    if zero == true{
                        self.state = CpuState::Fetch;
                    }
                    else {
                        let parsed_type = FromPrimitive::from_i8(Converter::bin_to_dec_pos_only(mdr_data[0..4].to_vec()).to_string().parse().unwrap());

                        if parsed_type.is_none() { self.decoded_instruction.instr_type = InstrType::OTH; } else { self.decoded_instruction.instr_type = parsed_type.unwrap(); }

                        match self.decoded_instruction.instr_type {
                            InstrType::ADD | InstrType::SUB | InstrType::MULT | InstrType::AND | InstrType::OR | InstrType::XOR => {
                                self.decoded_instruction.return_register.copy_from_slice(&mdr_data[4..8]);
                                self.decoded_instruction.reg_0 = mdr_data[8];
                                self.decoded_instruction.input_val_0.copy_from_slice(&mdr_data[9..25]);
                                self.decoded_instruction.reg_1 = mdr_data[25];
                                self.decoded_instruction.input_val_1.copy_from_slice(&mdr_data[26..42]);
                            }

                            InstrType::NOT | InstrType::FLIP => {
                                self.decoded_instruction.return_register.copy_from_slice(&mdr_data[4..8]);
                                self.decoded_instruction.reg_0 = mdr_data[8];
                                self.decoded_instruction.input_val_0.copy_from_slice(&mdr_data[9..25]);
                            }

                            InstrType::CMP => {
                                self.decoded_instruction.reg_0 = mdr_data[4];
                                self.decoded_instruction.input_val_0.copy_from_slice(&mdr_data[5..21]);
                                self.decoded_instruction.reg_1 = mdr_data[21];
                                self.decoded_instruction.input_val_1.copy_from_slice(&mdr_data[22..38]);
                            }

                            InstrType::STR | InstrType::LDR => {
                                self.decoded_instruction.return_register.copy_from_slice(&mdr_data[4..8]);
                                let mut addr_data: [bool; 48] = [false; 48];
                                addr_data.copy_from_slice(&mdr_data[8..56]);
                                self.decoded_instruction.addr = addr_data;
                            }

                            InstrType::B => {
                                self.decoded_instruction.branch_condition = FromPrimitive::from_u64(Converter::bin_to_dec_pos_only(mdr_data[4..8].to_vec())).unwrap();
                                let mut addr_data: [bool; 48] = [false; 48];
                                self.decoded_instruction.reg_0 = mdr_data[8];
                                addr_data.copy_from_slice(&mdr_data[9..57]);
                                self.decoded_instruction.addr = addr_data;
                            }

                            InstrType::OUT => {
                                self.decoded_instruction.return_register.copy_from_slice(&mdr_data[4..8]);
                                self.decoded_instruction.ascii = mdr_data[8];
                            }

                            _ => {}
                        }
                        self.state = CpuState::Execute;
                    }
                },

                CpuState::Execute => {

                    // Execute instruction based on intermediate representation, calling on relevant cpu components

                    match self.decoded_instruction.instr_type.clone() {
                        InstrType::ADD|InstrType::SUB|InstrType::MULT|InstrType::AND|InstrType::OR|InstrType::XOR => {

                            let mut val0 : [bool; 64] = [false; 64];
                            if self.decoded_instruction.reg_0{
                                let mut reg_index = [false; 4];
                                reg_index.copy_from_slice(&self.decoded_instruction.input_val_0.clone()[0..4]);
                                val0 = Converter::set_size(self.register_bank.get_data(reg_index).to_vec(), 64).try_into().unwrap();
                            }
                            else{
                                val0 = Converter::set_size(self.decoded_instruction.input_val_0.to_vec(), 64).try_into().unwrap();
                            }
                            let mut val1 : [bool; 64] = [false; 64];
                            if self.decoded_instruction.reg_1{
                                let mut reg_index = [false; 4];
                                reg_index.copy_from_slice(&self.decoded_instruction.input_val_1[0..4]);
                                val1 = Converter::set_size(self.register_bank.get_data(reg_index).to_vec(), 64).try_into().unwrap();
                            }
                            else{
                                val1 = Converter::set_size(self.decoded_instruction.input_val_1.to_vec(), 64).try_into().unwrap();
                            }

                            match self.decoded_instruction.instr_type.clone() {
                                InstrType::ADD => {
                                    self.register_bank.set_data(self.decoded_instruction.return_register, self.alu.add(val0, val1, false).0);
                                },
                                InstrType::SUB => {
                                    self.register_bank.set_data(self.decoded_instruction.return_register, self.alu.sub(val0, val1).0);
                                },
                                InstrType::MULT => {
                                    self.register_bank.set_data(self.decoded_instruction.return_register, self.alu.mult(val0, val1));
                                },
                                InstrType::AND | InstrType::OR | InstrType::XOR => { // Bitwise Operations
                                    let op_bits : [bool; 4] = Converter::dec_to_bin_pos_only(self.decoded_instruction.instr_type.clone() as u64, 4).try_into().unwrap();
                                    self.register_bank.set_data(self.decoded_instruction.return_register, self.alu.bitwise(val0, val1, op_bits));
                                },
                                _ => {}
                            }
                            self.state = CpuState::Fetch;
                        },

                        InstrType::NOT => { // Performs a bitwise NOT

                            let mut val0 : [bool; 64] = [false; 64];
                            if self.decoded_instruction.reg_0{
                                let mut reg_index = [false; 4];
                                reg_index.copy_from_slice(&self.decoded_instruction.input_val_0.clone()[0..4]);
                                val0 = Converter::set_size(self.register_bank.get_data(reg_index).to_vec(), 64).try_into().unwrap();
                            }
                            else{
                                val0 = Converter::set_size(self.decoded_instruction.input_val_0.to_vec(), 64).try_into().unwrap();
                            }

                            let op_bits : [bool; 4] = Converter::dec_to_bin_pos_only(self.decoded_instruction.instr_type.clone() as u64, 4).try_into().unwrap();
                            self.register_bank.set_data(self.decoded_instruction.return_register, self.alu.bitwise(val0, [false; 64], op_bits));
                            self.state = CpuState::Fetch;
                        },

                        InstrType::FLIP => { // FLips the value between positive and negative

                            let mut val0 : [bool; 64] = [false; 64];
                            if self.decoded_instruction.reg_0{
                                let mut reg_index = [false; 4];
                                reg_index.copy_from_slice(&self.decoded_instruction.input_val_0.clone()[0..4]);
                                val0 = self.register_bank.get_data(reg_index);
                            }
                            else{
                                val0 = Converter::set_size(self.decoded_instruction.input_val_0.to_vec(), 64).try_into().unwrap();
                            }

                            let mut single_bit = [false; 64];
                            single_bit[0] = true;

                            if val0[val0.len() - 1] {
                                let flipped = self.alu.bitwise(val0, [false; 64], Converter::dec_to_bin_pos_only(InstrType::NOT as u64, 4).try_into().unwrap());
                                self.register_bank.set_data(self.decoded_instruction.return_register, self.alu.add(flipped, single_bit, false).0);
                            }
                            else{
                                let subtracted = self.alu.sub(val0, single_bit).0;
                                self.register_bank.set_data(self.decoded_instruction.return_register, self.alu.bitwise(subtracted, [false; 64], Converter::dec_to_bin_pos_only(InstrType::NOT as u64, 4).try_into().unwrap()));
                            }
                            self.state = CpuState::Fetch;
                        },

                        InstrType::STR => {
                            self.data_access_manager.write(self.decoded_instruction.addr, self.register_bank.get_data(self.decoded_instruction.return_register));
                            self.state = CpuState::Fetch;
                        },

                        InstrType::LDR => {
                            let (data, cache_hit) = self.data_access_manager.read(self.decoded_instruction.addr);
                            if !cache_hit{
                                self.state = CpuState::Stall;
                            }
                            else{
                                self.memory_data_reg.set_data(data);
                                self.register_bank.set_data(self.decoded_instruction.return_register, self.memory_data_reg.get_data());
                                self.state = CpuState::Fetch;
                            }

                        },

                        InstrType::B => {

                            // Branching instructions

                            let mut valid_branch = false;
                            match self.decoded_instruction.branch_condition.clone() {
                                BranchConditions::B => {valid_branch = true;}, // Branch Always
                                BranchConditions::BEQ => { if self.alu.z { valid_branch = true; } }, // Branch if Equal
                                BranchConditions::BNE => { if !self.alu.z { valid_branch = true; } }, // Branch if Not Equal
                                BranchConditions::BLT => { if self.alu.n { valid_branch = true; } }, // Branch if Less Than
                                BranchConditions::BGT => { if !self.alu.z && !self.alu.n { valid_branch = true; } }, // Branch if Greater Than
                                BranchConditions::BLE => { if self.alu.z || self.alu.n { valid_branch = true; } }, // Branch if Less Than or Equal
                                BranchConditions::BGE => { if self.alu.z || !self.alu.n { valid_branch = true; } }, // Branch if Greater Than or Equal
                                _ => {}
                            }
                            if valid_branch {

                                if self.decoded_instruction.reg_0{

                                    let mut reg_data: [bool; 4] = [false; 4];
                                    reg_data.copy_from_slice(&self.decoded_instruction.addr[0..4]);
                                    println!("{}", vec_to_str(reg_data.to_vec()));

                                    self.pc.set_data(self.register_bank.get_data(reg_data));
                                }
                                else{
                                    self.pc.set_data(Converter::bit48_to64(self.decoded_instruction.addr));
                                }
                                println!("{}", vec_to_str(self.pc.get_data().to_vec()));

                            }
                            self.state = CpuState::Fetch;
                        },

                        InstrType::HLT => {
                            // Stops the clock
                            self.halt = true;
                        },

                        InstrType::OUT => {

                            let output_val= self.register_bank.get_data(self.decoded_instruction.return_register);
                            println!("R{0} OUTPUT: {1}", Converter::bin_to_dec_2s_comp(self.decoded_instruction.return_register.to_vec()), Converter::bin_to_dec_2s_comp(output_val.to_vec()));
                            /*
                            if self.decoded_instruction.ascii && Converter::bin_to_dec_2s_comp(output_val.to_vec()) >= 0 && Converter::bin_to_dec_2s_comp(output_val.to_vec()) <= 128{
                                let mut file = File::open("./output.txt").expect("Output File Error");
                                //file.write(String::from(char::from_u32(Converter::bin_to_dec_2s_comp(output_val.to_vec()) as u32).unwrap()).as_ref()).expect("Error Writing Output");
                                self.state = CpuState::Fetch;
                            }
                            else{
                                let mut file = File::open("./output.txt").expect("Output File Error");
                                //file.write(Converter::bin_to_dec_2s_comp(output_val.to_vec()).to_string().as_ref()).expect("Error Writing Output");
                                self.state = CpuState::Fetch;
                            }
                            */
                            self.state = CpuState::Fetch;
                        },

                        _ => {}

                    }
                },

                CpuState::Stall => {
                    // Stall state, waiting for memory
                    let (ready, data_bits) = self.data_access_manager.stall_read();
                    if ready{
                        if self.memory_instr_stall{
                            // If waiting for an instruction
                            self.memory_instr_stall = false;
                            self.memory_instr_reg.set_data(data_bits);
                            self.state = CpuState::Decode;
                        }
                        else if self.memory_data_stall{
                            // If waiting for data
                            self.memory_data_stall = false;
                            self.memory_data_reg.set_data(data_bits);
                            self.state = CpuState::MemoryComp;
                        }
                    }
                },
                
                CpuState::MemoryComp => {
                    self.register_bank.set_data(self.decoded_instruction.return_register, self.memory_data_reg.get_data());
                    self.state = CpuState::Fetch;
                }
            }
        }
    }
}