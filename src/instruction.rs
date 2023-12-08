use wasm_bindgen::prelude::wasm_bindgen;

pub type Number = i32;

#[derive(Debug, PartialEq)]
pub enum Operation {
    LDM(Number),
    OUT,
    END,
}

#[wasm_bindgen]
#[derive(Debug, Default, PartialEq)]
pub struct Instruction {
    pub(crate) labels: Vec<String>,
    pub(crate) operation: Option<Operation>,
}
