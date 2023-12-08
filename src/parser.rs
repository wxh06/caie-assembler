use crate::instruction::{Address, Instruction, Number, Operand, Operation, Register};
use wasm_bindgen::prelude::wasm_bindgen;

pub fn parse_operand(string: &str) -> Operand {
    Operand::new(string)
}

pub fn parse_number(string: &str) -> Number {
    let prefix = &string[..1];
    let number = &string[1..];
    match prefix {
        "#" => Number::from_str_radix(number, 10).expect("Invalid denary number"),
        "B" => Number::from_str_radix(number, 2).expect("Invalid binary number"),
        "&" => Number::from_str_radix(number, 16).expect("Invalid hexadecimal number"),
        _ => panic!("Invalid number"),
    }
}

pub fn parse_register(string: &str) -> Register {
    match string {
        "ACC" => Register::ACC,
        "IX" => Register::IX,
        _ => panic!("Invalid register"),
    }
}

/// TODO: Label support
pub fn parse_address(string: &str) -> Address {
    string.parse().unwrap()
}

impl Operand {
    pub fn new(string: &str) -> Operand {
        let address: Result<Address, _> = string.parse();
        match address {
            Ok(address) => Operand::Address(address),
            Err(_) => Operand::Number(parse_number(string)),
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
