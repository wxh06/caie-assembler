use wasm_bindgen::prelude::wasm_bindgen;

use crate::instruction::{
    AbsoluteAddress, Address, Assembler, Instruction, Number, Operation, Register,
};
use crate::utils::set_panic_hook;

#[wasm_bindgen]
#[derive(Debug, Eq, PartialEq)]
pub struct RuntimeError {
    pub address: AbsoluteAddress,
    message: String,
}

#[wasm_bindgen]
impl RuntimeError {
    #[wasm_bindgen(getter)]
    pub fn message(&self) -> String {
        self.message.clone()
    }
}

#[wasm_bindgen]
impl Assembler {
    #[wasm_bindgen]
    pub fn execute(&mut self, start: AbsoluteAddress) -> Result<(), RuntimeError> {
        set_panic_hook();

        let mut pc = start;
        let mut acc: Number = 0;
        let mut ix: Number = 0;
        let mut cmp: Option<bool> = None;

        macro_rules! get_register {
            ($register:expr) => {
                *match $register {
                    Register::ACC => &mut acc,
                    Register::IX => &mut ix,
                }
            };
        }
        macro_rules! direct {
            ($address:expr) => {{
                let address = $address;
                self.get_data(address).ok_or_else(|| RuntimeError {
                    address: pc,
                    message: format!("Invalid data at address {:?}", address),
                })?
            }};
        }
        macro_rules! indirect {
            ($address:expr) => {
                direct!(&Address::Absolute(direct!($address) as AbsoluteAddress))
            };
        }
        macro_rules! lookup {
            ($address:expr) => {{
                let address = $address;
                self.get_absolute_address(address)
                    .ok_or_else(|| RuntimeError {
                        address: pc,
                        message: format!("Invalid address {:?}", address),
                    })?
            }};
        }
        macro_rules! equal {
            () => {{
                let r = cmp.ok_or_else(|| RuntimeError {
                    address: pc,
                    message: String::from("Missing compare instruction"),
                })?;
                cmp = None;
                r
            }};
        }
        macro_rules! jump {
            ($address:expr) => {{
                let address = $address;
                pc = self
                    .get_absolute_address(address)
                    .ok_or_else(|| RuntimeError {
                        address: pc,
                        message: format!("Invalid operation at address {:?}", address),
                    })?
                    - 1;
            }};
        }

        loop {
            let instruction = self.instructions.get(&pc);
            if let Some(Instruction::Operation(operation)) = &instruction {
                println!("{}: ACC {}, IX {}", pc, acc, ix);
                match operation {
                    Operation::LoadImmediate(number) => acc = *number,
                    Operation::LoadDirect(address) => acc = direct!(address),
                    Operation::LoadIndirect(address) => acc = indirect!(address),
                    Operation::LoadIndexed(address) => {
                        acc = direct!(&Address::Absolute(lookup!(address) + ix as AbsoluteAddress))
                    }
                    Operation::LoadRegister(number) => ix = *number,
                    Operation::Move(register) => get_register!(register) = acc,
                    Operation::Store(address) => {
                        self.instructions
                            .insert(lookup!(address), Instruction::Data(acc));
                    }
                    Operation::AddDirect(address) => acc += direct!(address),
                    Operation::AddImmediate(number) => acc += number,
                    Operation::SubtractDirect(address) => acc -= direct!(address),
                    Operation::SubtractImmediate(number) => acc -= number,
                    Operation::Increase(register) => get_register!(register) += 1,
                    Operation::Decrease(register) => get_register!(register) -= 1,
                    Operation::Jump(address) => jump!(address),
                    Operation::CompareDirect(address) => cmp = Some(acc == direct!(address)),
                    Operation::CompareImmediate(number) => cmp = Some(acc == *number),
                    Operation::CompareIndirect(address) => cmp = Some(acc == indirect!(address)),
                    Operation::JumpEqual(address) => {
                        if equal!() {
                            jump!(address)
                        }
                    }
                    Operation::JumpNotEqual(address) => {
                        if !equal!() {
                            jump!(address)
                        }
                    }
                    Operation::Input => todo!(),
                    Operation::Output => println!("{}", acc as char),
                    Operation::End => break,
                }
            }
            pc += 1;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Location;

    use super::*;

    #[test]
    fn exec() {
        let mut a = Assembler::from_csv(include_str!("9618_s21_qp_11.csv")).unwrap();
        println!("{:#?}", a);
        a.execute(200).unwrap();
    }

    #[test]
    fn err_indirect() {
        let mut a = Assembler::from_records(vec![Location {
            address: 0,
            label: String::from(""),
            opcode: String::from("LDI"),
            operand: String::from("0"),
        }])
        .unwrap();
        println!("{:#?}", a);
        assert_eq!(
            a.execute(0),
            Err(RuntimeError {
                address: 0,
                message: String::from("Invalid data at address Absolute(0)"),
            })
        );
    }
}
