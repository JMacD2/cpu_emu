pub(crate) mod caches{
    use std::collections::HashMap;
    use std::vec::Vec;
    use crate::adders::adders::AddSub64bit;
    use crate::converter::converter::Converter;
    use crate::main_memory::main_memory::MainMemory;
    use crate::memory_chips::memory_chips::DRAM;

    struct CachedObj{
        key: [bool; 48],
        value: [bool; 64]
    }

    pub(crate) struct DataAccessManager {
        pub(crate) l1_cache: L1Cache,
        pub(crate) l2_cache: L2Cache,
        pub(crate) main_memory: MainMemory
    }
    impl DataAccessManager {
        pub fn get_index(&self, key: [bool; 48]) -> (i32, i32){
            // Returns (index, cache (1/2))
            let l1_index = self.l1_cache.get_index(key);
            if l1_index != -1{
                return (l1_index, 1);
            }
            let l2_index = self.l2_cache.get_index(key);
            if l2_index != -1{
                return (l2_index, 2);
            }
            (-1, -1)
        }

        pub fn read(&mut self, key : [bool; 48]) -> ([bool; 64], bool){
            let (index, level) = self.get_index(key);
            match level{
                1 => {
                    self.l1_cache.get_value(index)
                }
                2 => {
                    self.l2_cache.get_value(index)
                }
                _ => {

                    while self.main_memory.control_bus.lock {}
                    self.main_memory.control_bus.lock = true;

                    self.main_memory.address_bus.bits = key;
                    self.main_memory.control_bus.str = false;
                    self.main_memory.control_bus.ready_memory = true;

                    self.main_memory.control_bus.lock = false;

                    ([false; 64], false)
                }
            }
        }

        pub fn insert_to_cache(&mut self, key : [bool; 48], val : [bool; 64]){
            let (index, level) = self.get_index(key);
            match level{
                1 => {
                    self.l1_cache.insert(key, val);
                }
                2 => {
                    self.l2_cache.flush_loc(index);
                    let (obj, valid) = self.l1_cache.insert(key, val);
                    if valid{
                        self.l2_cache.insert(obj.key, obj.value);
                    }
                }
                _ => {
                    let (obj, valid) = self.l1_cache.insert(key, val);
                    if valid{
                        self.l2_cache.insert(obj.key, obj.value);
                    }
                }
            }
        }

        pub fn write(&mut self, key : [bool; 48], val : [bool; 64]){
            self.insert_to_cache(key, val);

            while self.main_memory.control_bus.lock {};
            self.main_memory.control_bus.lock = true;

            self.main_memory.address_bus.bits = key;
            self.main_memory.data_bus.bits = val;
            self.main_memory.control_bus.str = true;
            self.main_memory.control_bus.ready_memory = true;

            self.main_memory.control_bus.lock = false;
        }

        pub fn stall_read(&mut self) -> (bool, [bool; 64]){

            while self.main_memory.control_bus.lock {}
            self.main_memory.control_bus.lock = true;

            let ready = self.main_memory.control_bus.ready_cpu;
            let mut return_addr = [false; 48];
            let mut return_data = [false; 64];
            if ready{
                self.main_memory.control_bus.ready_cpu = false;
                return_addr = self.main_memory.address_bus.bits;
                return_data = self.main_memory.data_bus.bits;
                self.insert_to_cache(return_addr, return_data);
                self.main_memory.address_bus.bits = [false; 48];
                self.main_memory.data_bus.bits = [false; 64];
            }
            self.main_memory.control_bus.lock = false;

            (ready, return_data)
        }
    }
    
    
    
    struct TranslationLookasideBuffer {
        pub(crate) address_map: HashMap<[bool; 32], [bool; 32]>
    }
    impl TranslationLookasideBuffer{
        
    }
    impl Default for TranslationLookasideBuffer{
        fn default() -> Self {
            TranslationLookasideBuffer{
                address_map: Default::default(),
            }
        }
    }
    
    


    struct CacheQueue {
        add_sub64bit: AddSub64bit,
        lru_queue: Vec<CachedObj>,
        max_size: usize
    }
    impl CacheQueue {
        pub fn get_index(&self, key: [bool; 48]) -> i32{
            for i in 0..self.lru_queue.len(){
                let current_val: [bool; 64] = Converter::bit48_to64(self.lru_queue.get(i).clone().unwrap().key);
                let difference: [bool; 64] = self.add_sub64bit.value(current_val, Converter::bit48_to64(key), false).0;
                if Converter::bin_to_dec_2s_comp(difference.to_vec()) == 0{
                    return i as i32;
                }
            }
            -1
        }

        pub fn flush_loc(&mut self, index : i32){
            self.lru_queue.remove(index as usize);
        }

        pub fn get_value(&mut self, index : i32) -> ([bool; 64], bool){
            if index != -1{
                let key: [bool; 48] = self.lru_queue.get(index as usize).clone().unwrap().key;
                let val: [bool; 64] = self.lru_queue.get(index as usize).clone().unwrap().value;
                self.lru_queue.remove(index as usize);
                self.lru_queue.push(CachedObj{key: key.clone(), value: val.clone()});
                return (val, true);
            }
            ([false; 64], false)
        }

        pub fn insert(&mut self, key : [bool; 48], val : [bool; 64]) -> (CachedObj, bool){
            let index = self.get_index(key.clone());
            if index != -1{
                self.lru_queue.remove(index as usize);
                self.lru_queue.push(CachedObj{key: key.clone(), value: val.clone()});
            }
            else{
                if self.lru_queue.len() == self.max_size{
                    let flow_key = self.lru_queue.get(0).unwrap().key.clone();
                    let flow_val = self.lru_queue.get(0).unwrap().value.clone();
                    self.lru_queue.remove(0);
                    return (CachedObj{key: flow_key, value: flow_val}, true);

                }
                self.lru_queue.push(CachedObj{key: key.clone(), value: val.clone()});
            }
            (CachedObj{ key : [false; 48], value: [false; 64] }, false)
        }
    }


    pub(crate) struct L1Cache {
        cache_queue: CacheQueue
    }
    impl L1Cache{
        pub fn get_index(&self, key: [bool; 48]) -> i32{ self.cache_queue.get_index(key) }

        pub fn flush_loc(&mut self, index: i32){ self.cache_queue.flush_loc(index); }

        pub fn get_value(&mut self, index : i32) -> ([bool; 64], bool){ self.cache_queue.get_value(index) }

        pub fn insert(&mut self, key : [bool; 48], val : [bool; 64]) -> (CachedObj, bool){ self.cache_queue.insert(key, val) }
    }
    impl Default for L1Cache{
        fn default() -> Self {
            L1Cache{
                cache_queue: CacheQueue {
                    add_sub64bit: Default::default(),
                    lru_queue: vec![],
                    max_size: 20
                },
            }
        }
    }


    pub(crate) struct L2Cache {
        cache_queue: CacheQueue
    }
    impl L2Cache{
        pub fn get_index(&self, target : [bool; 48]) -> i32{ self.cache_queue.get_index(target) }

        pub fn flush_loc(&mut self, index : i32){ self.cache_queue.flush_loc(index); }

        pub fn get_value(&mut self, index : i32) -> ([bool; 64], bool){ self.cache_queue.get_value(index) }

        pub fn insert(&mut self, key : [bool; 48], val : [bool; 64]) -> (CachedObj, bool){ self.cache_queue.insert(key, val) }
    }

    impl Default for L2Cache{
        fn default() -> Self {
            L2Cache{
                cache_queue: CacheQueue {
                    add_sub64bit: Default::default(),
                    lru_queue: vec![],
                    max_size: 50
                },
            }
        }
    }

}