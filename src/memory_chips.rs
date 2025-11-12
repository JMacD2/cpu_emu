pub(crate) mod memory_chips {
    use std::time::{SystemTime, UNIX_EPOCH};

    #[derive(Copy, Clone)]
    pub(crate) struct DRAM {
        pub(crate) charge: bool,
        pub(crate) refresh_timer: u128
    }
    impl DRAM {
        pub fn current_time() -> u128 {
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
        }

        pub fn charge(&mut self) {
            self.refresh_timer = Self::current_time() + 64;
            self.charge = true;
        }

        pub fn dis_charge(&mut self) {
            self.charge = false;
        }

        pub fn read(&mut self) -> bool {
            /*
            if (self.refresh_timer < Self::current_time()) {
                self.charge = false;
                return false;
            }
             */
            if self.charge {
                self.dis_charge();
                self.charge();
                return true;
            }
            false
        }
    }

    impl Default for DRAM {
        fn default() -> Self {
            DRAM{ charge : false, refresh_timer: 0 }
        }
    }




    #[derive(Copy, Clone)]
    pub(crate) struct SRAM {
        pub(crate) charge: bool
    }
    impl SRAM {

        pub fn charge(&mut self) {
            self.charge = true;
        }

        pub fn discharge(&mut self) {
            self.charge = false;
        }
    }

    impl Default for SRAM {
        fn default() -> Self {
            SRAM {charge: false}
        }
    }
}