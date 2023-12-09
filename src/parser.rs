use wasm_bindgen::prelude::wasm_bindgen;

use crate::instruction::{AbsoluteAddress, Address, Instruction, Number, Operation, Register};
use crate::utils::set_panic_hook;
use crate::Assembly;

trait OperandParser {
    fn parse(string: &str) -> Self;
}

enum Operand {
    Number(Number),
    Address(Address),
}

impl OperandParser for Number {
    fn parse(string: &str) -> Self {
        let prefix = &string[..1];
        let number = &string[1..];
        match prefix {
            "#" => Self::from_str_radix(number, 10).expect("Invalid denary number"),
            "B" => Self::from_str_radix(number, 2).expect("Invalid binary number"),
            "&" => Self::from_str_radix(number, 16).expect("Invalid hexadecimal number"),
            _ => panic!("Invalid number"),
        }
    }
}

impl OperandParser for Address {
    fn parse(string: &str) -> Self {
        let absolute = string.parse();
        match absolute {
            Ok(address) => Self::Absolute(address),
            Err(_) => Self::Symbolic(String::from(string)),
        }
    }
}

impl OperandParser for Operand {
    fn parse(string: &str) -> Self {
        let number = string.parse();
        match number {
            Ok(number) => Self::Number(number),
            Err(_) => Self::Address(Address::parse(string)),
        }
    }
}

impl OperandParser for Register {
    fn parse(string: &str) -> Self {
        match string {
            "ACC" => Self::ACC,
            "IX" => Self::IX,
            _ => panic!("Invalid register"),
        }
    }
}

impl Operation {
    fn parse<'a>(opcode: &str, operands: &mut impl Iterator<Item = &'a str>) -> Self {
        let mut next_operand = || {
            operands
                .next()
                .expect(&format!("Too few operands to {}", opcode))
        };
        let operation = match opcode {
            "LDM" => Self::LoadImmediate(Number::parse(next_operand())),
            "LDD" => Self::LoadDirect(Address::parse(next_operand())),
            "LDI" => Self::LoadIndirect(Address::parse(next_operand())),
            "LDX" => Self::LoadIndexed(Address::parse(next_operand())),
            "LDR" => Self::LoadRegister(Number::parse(next_operand())),
            "MOV" => Self::Move(Register::parse(next_operand())),
            "STO" => Self::Store(Address::parse(next_operand())),
            "ADD" => match Operand::parse(next_operand()) {
                Operand::Address(address) => Self::AddDirect(address),
                Operand::Number(number) => Self::AddImmediate(number),
            },
            "SUB" => match Operand::parse(next_operand()) {
                Operand::Address(address) => Self::SubtractDirect(address),
                Operand::Number(number) => Self::SubtractImmediate(number),
            },
            "INC" => Self::Increase(Register::parse(next_operand())),
            "DEC" => Self::Decrease(Register::parse(next_operand())),
            "JMP" => Self::Jump(Address::parse(next_operand())),
            "CMP" => match Operand::parse(next_operand()) {
                Operand::Address(address) => Self::CompareDirect(address),
                Operand::Number(number) => Self::CompareImmediate(number),
            },
            "CMI" => Self::CompareIndirect(Address::parse(next_operand())),
            "JPE" => Self::JumpEqual(Address::parse(next_operand())),
            "JPN" => Self::JumpNotEqual(Address::parse(next_operand())),
            "IN" => Self::Input,
            "OUT" => Self::Output,
            "END" => Self::End,
            _ => panic!("Unknown opcode `{}`", opcode),
        };
        if operands.next().is_some() {
            panic!("Too many operands to {}", opcode)
        }
        operation
    }
}

impl Instruction {
    fn parse(string: &str) -> Self {
        set_panic_hook();
        let mut instruction: Self = Default::default();
        let mut split = string.split_ascii_whitespace();
        for segment in split.by_ref() {
            if segment.ends_with(':') {
                instruction
                    .labels
                    .push(String::from(&segment[0..(segment.len() - 1)]));
                continue;
            }
            instruction.operation = Some(Operation::parse(segment, &mut split));
            break;
        }
        instruction
    }
}

#[wasm_bindgen]
impl Assembly {
    #[wasm_bindgen(constructor)]
    pub fn new(source: &str, offset: AbsoluteAddress) -> Self {
        Self {
            instructions: Self::parse(&mut source.split_terminator("\n")),
            offset,
        }
    }
    fn parse<'a>(instructions: &mut impl Iterator<Item = &'a str>) -> Vec<Instruction> {
        instructions
            .map(|instruction| Instruction::parse(instruction))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ldm() {
        assert_eq!(
            Instruction::parse("A: B: LDM &FF"),
            Instruction {
                labels: vec![String::from("A"), String::from("B")],
                operation: Some(Operation::LoadImmediate(255)),
            }
        );
    }

    #[test]
    #[should_panic(expected = "Too many operands to END")]
    fn parse_end() {
        Instruction::parse("END #0");
    }

    #[test]
    #[should_panic(expected = "Unknown opcode `UNKNOWN`")]
    fn parse_unknown() {
        Instruction::parse("UNKNOWN");
    }
}
