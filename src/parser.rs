use wasm_bindgen::prelude::wasm_bindgen;

use crate::instruction::{AbsoluteAddress, Address, Instruction, Number, Operation, Register};
use crate::utils::set_panic_hook;
use crate::Assembly;

trait OperandParser {
    fn parse(string: &str) -> Result<Self, String>
    where
        Self: Sized;
}

enum Operand {
    Number(Number),
    Address(Address),
}

impl OperandParser for Number {
    fn parse(string: &str) -> Result<Self, String> {
        let prefix = &string[..1];
        let number = &string[1..];
        match prefix {
            "#" => Self::from_str_radix(number, 10)
                .map_err(|_| format!("Invalid denary number `{}`", number)),
            "B" => Self::from_str_radix(number, 2)
                .map_err(|_| format!("Invalid binary number `{}`", number)),
            "&" => Self::from_str_radix(number, 16)
                .map_err(|_| format!("Invalid hexadecimal number `{}`", number)),
            _ => Err(format!("Invalid number `{}`", string)),
        }
    }
}

impl OperandParser for Address {
    fn parse(string: &str) -> Result<Self, String> {
        let absolute = string.parse();
        match absolute {
            Ok(address) => Ok(Self::Absolute(address)),
            Err(_) => Ok(Self::Symbolic(String::from(string))),
        }
    }
}

impl OperandParser for Operand {
    fn parse(string: &str) -> Result<Self, String> {
        match Number::parse(string) {
            Ok(number) => Ok(Self::Number(number)),
            Err(_) => match Address::parse(string) {
                Ok(address) => Ok(Self::Address(address)),
                Err(_) => Err(format!("Invalid operand `{}`", string)),
            },
        }
    }
}

impl OperandParser for Register {
    fn parse(string: &str) -> Result<Self, String> {
        match string {
            "ACC" => Ok(Self::ACC),
            "IX" => Ok(Self::IX),
            _ => Err(format!("Invalid register `{}`", string)),
        }
    }
}

impl Operation {
    fn parse<'a>(
        opcode: &str,
        operands: &mut impl Iterator<Item = &'a str>,
    ) -> Result<Self, String> {
        macro_rules! operand {
            ($ty:ident) => {
                $ty::parse(
                    operands
                        .next()
                        .ok_or_else(|| format!("Too few operands to `{}`", opcode))?,
                )?
            };
        }
        macro_rules! operation {
            ($op:ident, $ty:ident) => {
                Self::$op(operand!($ty))
            };
        }

        let operation = match opcode {
            "LDM" => operation!(LoadImmediate, Number),
            "LDD" => operation!(LoadDirect, Address),
            "LDI" => operation!(LoadIndirect, Address),
            "LDX" => operation!(LoadIndexed, Address),
            "LDR" => operation!(LoadRegister, Number),
            "MOV" => operation!(Move, Register),
            "STO" => operation!(Store, Address),
            "ADD" => match operand!(Operand) {
                Operand::Address(address) => Self::AddDirect(address),
                Operand::Number(number) => Self::AddImmediate(number),
            },
            "SUB" => match operand!(Operand) {
                Operand::Address(address) => Self::SubtractDirect(address),
                Operand::Number(number) => Self::SubtractImmediate(number),
            },
            "INC" => operation!(Increase, Register),
            "DEC" => operation!(Decrease, Register),
            "JMP" => operation!(Jump, Address),
            "CMP" => match operand!(Operand) {
                Operand::Address(address) => Self::CompareDirect(address),
                Operand::Number(number) => Self::CompareImmediate(number),
            },
            "CMI" => operation!(CompareIndirect, Address),
            "JPE" => operation!(JumpEqual, Address),
            "JPN" => operation!(JumpNotEqual, Address),
            "IN" => Self::Input,
            "OUT" => Self::Output,
            "END" => Self::End,
            _ => return Err(format!("Unknown opcode `{}`", opcode)),
        };
        if operands.next().is_some() {
            return Err(format!("Too many operands to `{}`", opcode));
        }
        Ok(operation)
    }
}

impl Instruction {
    fn parse(string: &str) -> Result<Self, String> {
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
            instruction.operation = Some(Operation::parse(segment, &mut split)?);
            break;
        }
        Ok(instruction)
    }
}

#[wasm_bindgen]
impl Assembly {
    #[wasm_bindgen(constructor)]
    pub fn new(source: &str, offset: AbsoluteAddress) -> Result<Assembly, String> {
        Ok(Self {
            instructions: Self::parse(&mut source.split_terminator('\n'))?,
            offset,
        })
    }
    fn parse<'a>(
        instructions: &mut impl Iterator<Item = &'a str>,
    ) -> Result<Vec<Instruction>, String> {
        instructions.map(Instruction::parse).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ldm() {
        assert_eq!(
            Instruction::parse("A: B: LDM &FF"),
            Ok(Instruction {
                labels: vec![String::from("A"), String::from("B")],
                operation: Some(Operation::LoadImmediate(255)),
            })
        );
    }

    #[test]
    fn parse_ldr() {
        assert_eq!(
            Instruction::parse("LDR B2"),
            Err(String::from("Invalid binary number `2`"))
        );
    }

    #[test]
    fn parse_end() {
        assert_eq!(
            Instruction::parse("END #0"),
            Err(String::from("Too many operands to `END`"))
        );
    }

    #[test]
    fn parse_unknown() {
        assert_eq!(
            Instruction::parse("UNKNOWN"),
            Err(String::from("Unknown opcode `UNKNOWN`"))
        );
    }
}
