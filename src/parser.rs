use crate::instruction::{Address, Instruction, Number, Operand, Operation};
use wasm_bindgen::prelude::wasm_bindgen;

impl Operand {
    fn new(string: &str) -> Operand {
        let address: Result<Address, _> = string.parse();
        match address {
            Ok(address) => Operand::Address(address),
            Err(_) => Operand::Number(Self::parse_number(string)),
        }
    }
    fn parse_number(string: &str) -> Number {
        let prefix = &string[..1];
        let number = &string[1..];
        match prefix {
            "#" => Number::from_str_radix(number, 10).expect("Invalid denary number"),
            "B" => Number::from_str_radix(number, 2).expect("Invalid binary number"),
            "&" => Number::from_str_radix(number, 16).expect("Invalid hexadecimal number"),
            _ => panic!("Invalid number"),
        }
    }
    /// TODO: Label support
    fn parse_address(string: &str) -> Address {
        string.parse().unwrap()
    }
}

impl Operation {
    fn new<'a>(opcode: &str, operand: &mut impl Iterator<Item = &'a str>) -> Operation {
        let mut next_operand = || {
            operand
                .next()
                .expect(format!("Too few operands to {}", opcode).as_ref())
        };
        match opcode {
            "LDM" => Operation::LDM(Operand::parse_number(next_operand())),
            "LDD" => Operation::LDD(Operand::parse_address(next_operand())),
            "ADD" => Operation::ADD(Operand::new(next_operand())),
            "OUT" => Operation::OUT,
            "END" => Operation::END,
            _ => panic!("Unknown opcode"),
        }
    }
}

#[wasm_bindgen]
impl Instruction {
    #[wasm_bindgen(constructor)]
    pub fn new(string: &str) -> Instruction {
        let mut instruction: Instruction = Default::default();
        let mut split = string.split_ascii_whitespace();
        for segment in split.by_ref() {
            if segment.ends_with(":") {
                instruction
                    .labels
                    .push(String::from(&segment[0..(segment.len() - 1)]));
                continue;
            }
            instruction.operation = Some(Operation::new(segment, &mut split));
            break;
        }
        if split.next().is_some() {
            panic!("Too many operands")
        }
        return instruction;
    }
}

pub fn parse(instructions: Vec<&str>) {
    for instruction in instructions {
        println!("{:?}", Instruction::new(instruction));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ldm() {
        assert_eq!(
            Instruction::new("A: B: LDM &FF"),
            Instruction {
                labels: vec![String::from("A"), String::from("B")],
                operation: Some(Operation::LDM(255)),
            }
        );
    }

    #[test]
    #[should_panic(expected = "Too many operands")]
    fn parse_end() {
        Instruction::new("END #0");
    }

    #[test]
    #[should_panic(expected = "Unknown opcode")]
    fn parse_unknown() {
        Instruction::new("UNKNOWN");
    }
}
