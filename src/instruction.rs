use std::collections::HashMap;

use wasm_bindgen::prelude::wasm_bindgen;

pub type Number = u8;
pub type AbsoluteAddress = usize;

#[derive(Debug, Eq, PartialEq)]
pub enum Address {
    Absolute(AbsoluteAddress),
    Symbolic(String),
}

#[derive(Debug, Eq, PartialEq)]
pub enum Register {
    ACC,
    IX,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Operation {
    LoadImmediate(Number),
    LoadDirect(Address),
    LoadIndirect(Address),
    LoadIndexed(Address),
    LoadRegister(Number),
    Move(Register),
    Store(Address),
    AddDirect(Address),
    AddImmediate(Number),
    SubtractDirect(Address),
    SubtractImmediate(Number),
    Increase(Register),
    Decrease(Register),
    Jump(Address),
    CompareDirect(Address),
    CompareImmediate(Number),
    CompareIndirect(Address),
    JumpEqual(Address),
    JumpNotEqual(Address),
    Input,
    Output,
    End,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Instruction {
    Operation(Operation),
    Data(Number),
}

pub type SymbolTable = HashMap<String, AbsoluteAddress>;
pub type Instructions = HashMap<AbsoluteAddress, Instruction>;

#[wasm_bindgen]
#[derive(Debug, Eq, PartialEq)]
pub struct Assembler {
    pub(crate) instructions: Instructions,
    pub(crate) symbol_table: SymbolTable,
}
