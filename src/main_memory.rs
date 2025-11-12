pub(crate) mod main_memory {
    use crate::memory_chips::memory_chips::DRAM;
    use std::collections::HashMap;
    use crate::buses::buses::{AddressBus, ControlBus, DataBus};
    use crate::converter::converter::Converter;

    // A HashMap is simply used to reduce to RAM requirements of having a full memory array stored
    // Stores values in individual 'DRAM' chips, rather than as just booleans
    pub(crate) struct MainMemory {
        pub(crate) ram_map: HashMap<[bool; 48], [DRAM; 64]>,
        pub(crate) data_bus: DataBus,
        pub(crate) address_bus: AddressBus,
        pub(crate) control_bus: ControlBus
    }

    impl MainMemory {

        pub fn get_valid_start(loc : [bool; 48]) -> [bool; 48] {
            // Moves addresses to the valid 64-bit word start
            if Converter::bin_to_dec_pos_only(loc.to_vec()) % 64 == 0 {
                loc
            } else {
                let round_up = (Converter::bin_to_dec_pos_only(loc.to_vec()) as f64 / 64.0).ceil() as u64;
                Converter::dec_to_bin_pos_only(round_up, 48).try_into().unwrap()
            }
        }

        pub fn write(&mut self, loc : [bool; 48], val : [bool; 64]) {
            let mut dram_write = [DRAM::default(); 64];
            for i in 0..dram_write.len() {
                if val[i]{
                    dram_write[i].charge();
                }
            }
            self.ram_map.insert(MainMemory::get_valid_start(loc), dram_write);
        }

        pub fn read(&mut self, loc : [bool; 48]) -> [bool; 64] {
            let mut return_bits = [false; 64];
            let dram_read = self.ram_map.get(&MainMemory::get_valid_start(loc)).cloned().unwrap_or([DRAM::default(); 64]).clone();
            for i in 0..dram_read.len() {
                if dram_read[i].charge{
                    return_bits[i] = true;
                }
            }
            return_bits
        }


        pub fn clear(&mut self) {
            self.ram_map.clear();
        }

        pub fn tick(&mut self){ // Controls interaction with buses
            while self.control_bus.lock {}
            self.control_bus.lock = true;
            if self.control_bus.ready_memory{
                self.control_bus.ready_memory = false;
                if self.control_bus.str{
                    self.write(self.address_bus.bits, self.data_bus.bits);
                    self.control_bus.str = false;
                }
                else{
                    self.data_bus.bits = self.read(self.address_bus.bits);
                    self.control_bus.ready_cpu = true;
                }
            }
            self.control_bus.lock = false;
        }
    }
}

