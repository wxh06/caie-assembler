use wasm_bindgen::prelude::wasm_bindgen;

pub type Number = i32;
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

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Instruction {
    pub labels: Vec<String>,
    pub operation: Option<Operation>,
}

#[wasm_bindgen]
pub struct Assembly {
    pub(crate) instructions: Vec<Instruction>,
    pub(crate) offset: AbsoluteAddress,
}
