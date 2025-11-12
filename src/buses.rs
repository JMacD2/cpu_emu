pub(crate) mod buses{
    pub(crate) struct AddressBus {
        pub(crate) bits : [bool; 48],
    }
    impl AddressBus {}
    impl Default for AddressBus { fn default() -> Self { AddressBus { bits : [false; 48] } } }

    pub(crate) struct DataBus {
        pub(crate) bits : [bool; 64],
    }
    impl DataBus {}
    impl Default for DataBus { fn default() -> Self { DataBus { bits : [false; 64] } } }

    pub(crate) struct ControlBus {
        pub(crate) ready_memory : bool,
        pub(crate) ready_cpu : bool,
        pub(crate) str : bool,
        pub(crate) lock : bool
    }
    impl ControlBus {}
    impl Default for ControlBus {
        fn default() -> Self { ControlBus { ready_memory: false, ready_cpu: false, str: false, lock: false } }
    }
}