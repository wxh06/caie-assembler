use std::collections::HashMap;

type Address = usize;
type Number = i32;

enum Operand {
    Number(Number),
    Address(Address),
}

enum Instruction {
    Load(Operand),
    Store(Address),
    Add(Operand),
    Subtract(Operand),
    OUT,
}

struct Execution {
    instructions: Vec<Instruction>,
    memory: HashMap<Address, Number>,
    accumulator: Number,
}

impl Execution {
    pub fn new(instructions: Vec<Instruction>) -> Execution {
        Execution {
            instructions,
            memory: Default::default(),
            accumulator: 0,
        }
    }

    pub fn execute(&mut self) {
        let mut i = 0;
        while i < self.instructions.len() {
            let instruction = &self.instructions[i];
            match instruction {
                Instruction::Load(operand) => {
                    self.accumulator = *match operand {
                        Operand::Number(number) => number,
                        Operand::Address(address) => {
                            self.memory.get(&address).expect("invalid address")
                        }
                    }
                }
                Instruction::Store(address) => {
                    self.memory.insert(*address, self.accumulator);
                }
                Instruction::Add(operand) | Instruction::Subtract(operand) => {
                    let number = match operand {
                        Operand::Number(number) => number,
                        Operand::Address(address) => self.memory.get(&address).expect(""),
                    };
                    match instruction {
                        Instruction::Add(_) => self.accumulator += number,
                        Instruction::Subtract(_) => self.accumulator -= number,
                        _ => {}
                    }
                }
                Instruction::OUT => println!("{}", self.accumulator),
            }
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ldm() {
        let mut execution = Execution::new(vec![Instruction::Load(Operand::Number(1))]);
        execution.execute();
        assert_eq!(execution.accumulator, 1);
    }

    #[test]
    fn ldd() {
        let mut execution = Execution::new(vec![Instruction::Load(Operand::Address(2))]);
        execution.memory.insert(2, 1);
        execution.execute();
        assert_eq!(execution.accumulator, 1);
    }

    #[test]
    fn sto() {
        let mut execution = Execution::new(vec![Instruction::Store(0)]);
        execution.execute();
        assert_eq!(execution.memory.get(&0), Some(&0));
    }

    #[test]
    fn add_and_sub_number() {
        let mut execution = Execution::new(vec![
            Instruction::Add(Operand::Number(2)),
            Instruction::Subtract(Operand::Number(1)),
        ]);
        execution.execute();
        assert_eq!(execution.accumulator, 1);
    }

    #[test]
    fn add_address() {
        let mut execution = Execution::new(vec![Instruction::Add(Operand::Address(0))]);
        execution.memory.insert(0, 1);
        execution.execute();
        assert_eq!(execution.accumulator, 1);
    }
}
