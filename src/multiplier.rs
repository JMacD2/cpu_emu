pub(crate) mod multiplier{
    use crate::adders::adders::AddSub64bit;
    use crate::converter::converter::Converter;
    use crate::reg64::reg64::Reg64;

    pub(crate) struct Multiplier {
        adder : AddSub64bit,
        multiplier_acc : Reg64,
        multiplier_operand_store : Reg64,
        multiplier_counter : Reg64
    }
    impl Multiplier {

        pub fn value(&mut self, val0: [bool; 64], val1: [bool; 64]) -> [bool; 64]{
            self.multiplier_operand_store.set_data(val0);
            self.multiplier_counter.set_data(val1);
            let mut one_bit : [bool; 64] = [false; 64];
            one_bit[0] = true;

            let negative_multiplier = self.multiplier_counter.get_data()[self.multiplier_counter.get_data().len() - 1];
            if negative_multiplier{
                self.multiplier_counter.set_data(Converter::bin_flip_sign(self.multiplier_counter.get_data()));
            }

            while Converter::bin_to_dec_pos_only(self.multiplier_counter.get_data().to_vec()) > 0{
                self.multiplier_acc.set_data(self.adder.value(self.multiplier_acc.get_data(), self.multiplier_operand_store.get_data(), true).0);
                self.multiplier_counter.set_data(self.adder.value(self.multiplier_counter.get_data(), one_bit, false).0);
            }

            if negative_multiplier{
                self.multiplier_acc.set_data(Converter::bin_flip_sign(self.multiplier_acc.get_data()));
            }
            
            self.multiplier_acc.get_data()

        }

    }
    impl Default for Multiplier {
        fn default() -> Self {
            Multiplier {
                adder: Default::default(),
                multiplier_acc: Default::default(),
                multiplier_operand_store: Default::default(),
                multiplier_counter: Default::default()
            }
        }
    }
}