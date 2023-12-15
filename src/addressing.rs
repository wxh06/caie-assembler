use crate::instruction::{AbsoluteAddress, Address, Assembler, Instruction, Number};

impl Assembler {
    pub(crate) fn get_data(&self, address: &Address) -> Option<Number> {
        let absolute_address: AbsoluteAddress = match address {
            Address::Absolute(absolute_address) => *absolute_address,
            Address::Symbolic(symbolic_address) => *self.symbol_table.get(symbolic_address)?,
        };
        match self.instructions.get(&absolute_address)? {
            Instruction::Operation(_) => None,
            Instruction::Data(data) => Some(*data),
        }
    }

    pub(crate) fn get_absolute_address(&self, address: &Address) -> Option<AbsoluteAddress> {
        match address {
            Address::Absolute(absolute_address) => Some(*absolute_address),
            Address::Symbolic(symbolic_address) => self.symbol_table.get(symbolic_address).copied(),
        }
    }
}
