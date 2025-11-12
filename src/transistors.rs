pub(crate) mod transistors {

    #[derive(Clone, Copy)]
    pub(crate) struct Pmos {}
    impl Pmos {
        pub fn value(&self, input: bool, power: bool) -> bool {
            !input && power
        }
    }

    #[derive(Clone, Copy)]
    pub(crate) struct Nmos {}
    impl Nmos {
        pub fn value(&self, input: bool, power: bool) -> bool {
            input && power
        }
    }
}