use crate::parser::{parse_address, parse_number, parse_operand, parse_register};
use parser_derive::Parser;
use wasm_bindgen::prelude::wasm_bindgen;

pub type Number = i32;
pub type Address = usize;

#[derive(Debug, PartialEq)]
pub enum Register {
    ACC,
    IX,
}

#[derive(Debug, PartialEq)]
pub enum Operand {
    Number(Number),
    Address(Address),
}

#[derive(Debug, Parser, PartialEq)]
pub enum Operation {
    LDM(Number),
    LDD(Address),
    LDI(Address),
    LDX(Address),
    LDR(Number),
    MOV(Register),
    STO(Address),
    ADD(Operand),
    SUB(Operand),
    INC(Register),
    DEC(Register),
    JMP(Address),
    CMP(Operand),
    CMI(Address),
    JPE(Address),
    JPN(Address),
    IN,
    OUT,
    END,
}

#[wasm_bindgen]
#[derive(Debug, Default, PartialEq)]
pub struct Instruction {
    pub(crate) labels: Vec<String>,
    pub(crate) operation: Option<Operation>,
}
