use serde::Deserialize;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::instruction::{
    AbsoluteAddress, Address, Instruction, Instructions, Number, Operation, Register, SymbolTable,
};
use crate::utils::set_panic_hook;
use crate::Assembler;

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
    fn parse<'a>(opcode: &str, operand: &str) -> Result<Self, String> {
        macro_rules! operand {
            ($ty:ident) => {
                $ty::parse(operand)?
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
        Ok(operation)
    }
}

#[derive(Debug, Deserialize)]
struct Location {
    address: AbsoluteAddress,
    label: String,
    opcode: String,
    operand: String,
}

impl Instruction {
    fn from(opcode: &str, operand: &str) -> Result<Self, String> {
        Ok(if opcode.is_empty() {
            Self::Data(operand.parse().or_else(|_| Number::parse(operand))?)
        } else {
            Self::Operation(Operation::parse(opcode, operand)?)
        })
    }
}

#[wasm_bindgen]
impl Assembler {
    pub fn from_csv(data: &str) -> Result<Assembler, String> {
        set_panic_hook();
        let mut rdr = csv::Reader::from_reader(data.as_bytes());
        Self::from_records(
            rdr.deserialize()
                .map(|record| record.map_err(|_| String::from("")))
                .collect::<Result<_, _>>()?,
        )
    }
    fn from_records(records: Vec<Location>) -> Result<Self, String> {
        let mut symbol_table: SymbolTable = Default::default();
        let mut instructions: Instructions = Default::default();
        for record in records {
            symbol_table.insert(record.label, record.address);
            instructions.insert(
                record.address,
                Instruction::from(&record.opcode, &record.operand)?,
            );
        }
        Ok(Self {
            instructions,
            symbol_table,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ldm() {
        let a = Assembler::from_records(vec![Location {
            address: 0,
            label: String::from("L"),
            opcode: String::from("LDM"),
            operand: String::from("&FF"),
        }])
        .unwrap();
        assert_eq!(
            a.instructions.get(&0),
            Some(&Instruction::Operation(Operation::LoadImmediate(255)))
        );
        assert_eq!(a.symbol_table.get("L"), Some(&0));
    }

    #[test]
    fn parse_ldr() {
        assert_eq!(
            Instruction::from("LDR", "B2"),
            Err(String::from("Invalid binary number `2`"))
        );
    }

    #[test]
    fn parse_unknown() {
        assert_eq!(
            Instruction::from("UNKNOWN", ""),
            Err(String::from("Unknown opcode `UNKNOWN`"))
        );
    }
}
