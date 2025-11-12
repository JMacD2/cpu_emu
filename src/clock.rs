pub(crate) mod clock{
    use crate::control_unit::control_unit::ControlUnit;
    use crate::converter::converter::Converter;
    use crate::{read_pipe, send_memory};

    pub(crate) struct Clock {
        pub(crate) clock_speed : i64, // Frequency in Hz
        pub(crate) running : bool,
        pub(crate) ctrl : ControlUnit,

        pub(crate) cycle_count : u64
    }
    impl Clock {

        pub fn start(&mut self){
            self.running = true;
            while self.running {
                self.parse_pipe_data(read_pipe());
                self.refresh();
            }
        }

        pub fn stop(&mut self){
            self.running = false;
        }

        pub fn refresh(&mut self){
            self.running = !self.ctrl.halt;
            if self.running{
                self.cycle_count += 1;
                // AT PRESENT - both the CPU and RAM are controlled by the same clock - these could be separated in future for greater realism
                self.ctrl.tick();
                self.ctrl.data_access_manager.main_memory.tick();
            }
            else{
                println!("CYCLE COUNT: {0}", self.cycle_count); // Print total cycle count after completion
            }
        }

        fn parse_pipe_data(&mut self, data : String){
            // WORK-IN-PROGRESS GUI IMPLEMENTATION
            if data == "INC_CLK"{
                self.clock_speed += 1;
            }
            else if data == "DEC_CLK"{
                self.clock_speed -= 1;
            }
            else{
                let mut split_str = data.split("//");
                if split_str.clone().count() != 2{ return; }
                if split_str.clone().nth(0).unwrap() == "GET"{
                    let read_addr = Converter::hex_val_to_bin(split_str.clone().nth(1).unwrap().to_string());
                    let return_data_bin = self.ctrl.data_access_manager.main_memory.read(read_addr.try_into().unwrap());
                    send_memory(split_str.clone().nth(1).unwrap().to_string(), Converter::bin_to_hex(return_data_bin.to_vec()));
                }
                else if split_str.clone().nth(0).unwrap() == "SET_CLK"{
                    self.clock_speed = split_str.clone().nth(1).unwrap().to_string().parse().unwrap();
                }
            }
        }
    }
}