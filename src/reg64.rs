pub(crate) mod reg64{
    // Simply stores 64-bit values using simulated SRAM chips
    use crate::memory_chips::memory_chips::SRAM;

    #[derive(Clone, Copy)]
    pub(crate) struct Reg64 {
        pub(crate) data_cells : [SRAM; 64]
    }

    impl Reg64 {
        pub fn clear_data(&mut self){
            for i in 0..64{
                self.data_cells[i].discharge();
            }
        }

        pub fn set_data(&mut self, data : [bool; 64]){
            for i in 0..64{
                self.data_cells[i].discharge();
                if data[i]{
                    self.data_cells[i].charge();
                }
            }
        }

        pub fn get_data(&self) -> [bool; 64] {
            let mut return_data: [bool; 64] = [false; 64];
            for i in 0..64{
                if self.data_cells[i].charge{
                    return_data[i] = true;
                }
            }
            return_data
        }
    }

    impl Default for Reg64 {
        fn default() -> Self { Reg64{data_cells : [SRAM::default(); 64]} }
    }
}