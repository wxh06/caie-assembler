use std::collections::HashMap;

type Address = usize;
type Number = i32;

enum Instruction {
    LDM(Number),
    LDD(Address),
    STO(Address),
    OUT,
}

struct Execution {
    instructions: Vec<Instruction>,
    memory: HashMap<Address, Number>,
    acc: Number,
}

impl Execution {
    pub fn new(instructions: Vec<Instruction>) -> Execution {
        Execution {
            instructions,
            memory: Default::default(),
            acc: 0,
        }
    }

    pub fn execute(&mut self) {
        let mut i = 0;
        while i < self.instructions.len() {
            match self.instructions[i] {
                Instruction::LDM(number) => self.acc = number,
                Instruction::LDD(address) => {
                    self.acc = *self.memory.get(&address).expect("invalid address")
                }
                Instruction::STO(address) => {
                    self.memory.insert(address, self.acc);
                }
                Instruction::OUT => println!("{}", self.acc),
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
        let mut execution = Execution::new(vec![Instruction::LDM(1)]);
        execution.execute();
        assert_eq!(execution.acc, 1);
    }

    #[test]
    fn ldd() {
        let mut execution = Execution::new(vec![Instruction::LDD(2)]);
        execution.memory.insert(2, 1);
        execution.execute();
        assert_eq!(execution.acc, 1);
    }

    #[test]
    fn sto() {
        let mut execution = Execution::new(vec![Instruction::STO(0)]);
        execution.execute();
        assert_eq!(execution.memory.get(&0), Some(&0));
    }
}
