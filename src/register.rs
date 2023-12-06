use crate::operand::Number;

pub enum Register {
    ACC,
    IX,
}

#[derive(Default)]
pub struct Registers {
    pub accumulator: Number,
    pub index_register: Number,
}

impl Registers {
    pub fn get_register_mut(&mut self, register: &Register) -> &mut Number {
        match register {
            Register::ACC => &mut self.accumulator,
            Register::IX => &mut self.index_register,
        }
    }
}
