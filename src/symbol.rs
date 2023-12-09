use crate::instruction::{AbsoluteAddress, Address, Assembler, SymbolTable};

impl Assembler {
    pub fn generate_symbol_table(&mut self) {
        for (i, instruction) in self.instructions.iter().enumerate() {
            for label in &instruction.labels {
                self.symbol_table.insert(label.clone(), i + self.offset);
            }
        }
    }
}

impl Address {
    pub fn to_numeric(&self, symbol_table: SymbolTable) -> Result<AbsoluteAddress, String> {
        match self {
            Self::Symbolic(symbol) => symbol_table
                .get(symbol)
                .copied()
                .ok_or_else(|| format!("Unknown symbolic address `{}`", symbol)),
            Self::Absolute(address) => Ok(*address),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::instruction::{Instruction, Operation};

    use super::*;

    #[test]
    fn symbol_table() {
        let label = "label";
        let mut a = Assembler {
            instructions: vec![
                Instruction {
                    labels: vec![],
                    operation: Some(Operation::LoadDirect(Address::Absolute(2))),
                },
                Instruction {
                    labels: vec![String::from(label)],
                    operation: None,
                },
            ],
            offset: 1,
            symbol_table: Default::default(),
        };
        a.generate_symbol_table();
        match &a.instructions[0].operation {
            Some(Operation::LoadDirect(address)) => {
                assert_eq!(address.to_numeric(a.symbol_table), Ok(2));
            }
            _ => panic!(),
        };
    }
}
