use std::collections::HashMap;

type Address = usize;
type Number = i32;

enum Register {
    ACC,
    IX,
}

enum Operand {
    Number(Number),
    Address(Address),
}

enum Instruction {
    Load(Register, Operand),
    Move(Register),
    Store(Address),
    Add(Register, Operand),
    Subtract(Register, Operand),
    OUT,
}

struct Registers {
    accumulator: Number,
    index_register: Number,
}

impl Registers {
    pub fn get_register_mut(&mut self, register: &Register) -> &mut Number {
        match register {
            Register::ACC => &mut self.accumulator,
            Register::IX => &mut self.index_register,
        }
    }
}

struct Execution {
    instructions: Vec<Instruction>,
    memory: HashMap<Address, Number>,
    registers: Registers,
}

impl Execution {
    pub fn new(instructions: Vec<Instruction>) -> Execution {
        Execution {
            instructions,
            memory: Default::default(),
            registers: Registers {
                accumulator: 0,
                index_register: 0,
            },
        }
    }

    pub fn get_operand_value(&self, operand: &Operand) -> Number {
        *match operand {
            Operand::Number(number) => number,
            Operand::Address(address) => self.memory.get(&address).expect("invalid address"),
        }
    }

    pub fn execute(&mut self) {
        let mut i = 0;
        while i < self.instructions.len() {
            let instruction = &self.instructions[i];
            match instruction {
                Instruction::Load(register, operand) => {
                    *self.registers.get_register_mut(register) = self.get_operand_value(operand)
                }
                Instruction::Move(register) => {
                    *self.registers.get_register_mut(register) = self.registers.accumulator
                }
                Instruction::Store(address) => {
                    self.memory.insert(*address, self.registers.accumulator);
                }
                Instruction::Add(register, operand) => {
                    *self.registers.get_register_mut(register) += self.get_operand_value(operand)
                }
                Instruction::Subtract(register, operand) => {
                    *self.registers.get_register_mut(register) -= self.get_operand_value(operand)
                }
                Instruction::OUT => println!("{}", self.registers.accumulator),
            }
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ldm_and_mov() {
        let mut execution = Execution::new(vec![
            Instruction::Load(Register::ACC, Operand::Number(1)),
            Instruction::Move(Register::IX),
        ]);
        execution.execute();
        assert_eq!(execution.registers.accumulator, 1);
        assert_eq!(execution.registers.index_register, 1);
    }

    #[test]
    fn ldd_and_sto() {
        let mut execution = Execution::new(vec![
            Instruction::Load(Register::IX, Operand::Address(0)),
            Instruction::Store(1),
        ]);
        execution.memory.insert(0, 1);
        execution.execute();
        assert_eq!(execution.registers.index_register, 1);
        assert_eq!(execution.memory.get(&1), Some(&0));
    }

    #[test]
    fn add_and_sub_number() {
        let mut execution = Execution::new(vec![
            Instruction::Add(Register::IX, Operand::Number(1)),
            Instruction::Subtract(Register::ACC, Operand::Number(-1)),
        ]);
        execution.execute();
        assert_eq!(execution.registers.accumulator, 1);
        assert_eq!(execution.registers.index_register, 1);
    }

    #[test]
    fn add_address() {
        let mut execution =
            Execution::new(vec![Instruction::Add(Register::ACC, Operand::Address(0))]);
        execution.memory.insert(0, 1);
        execution.execute();
        assert_eq!(execution.registers.accumulator, 1);
    }
}
