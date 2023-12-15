use crate::instruction::{
    AbsoluteAddress, Address, Assembler, Instruction, Number, Operation, Register,
};
use crate::utils::set_panic_hook;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
impl Assembler {
    #[wasm_bindgen]
    pub fn execute(&mut self, start: AbsoluteAddress) {
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
            ($address:expr) => {
                self.get_data($address).unwrap()
            };
        }
        macro_rules! indirect {
            ($address:expr) => {
                direct!(&Address::Absolute(direct!($address) as AbsoluteAddress))
            };
        }

        macro_rules! jump {
            ($address:expr) => {
                pc = self.get_absolute_address($address).unwrap() - 1
            };
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
                        acc = direct!(&Address::Absolute(
                            self.get_absolute_address(address).unwrap() + ix as AbsoluteAddress,
                        ))
                    }
                    Operation::LoadRegister(number) => ix = *number,
                    Operation::Move(register) => get_register!(register) = acc,
                    Operation::Store(address) => {
                        self.instructions.insert(
                            self.get_absolute_address(address).unwrap(),
                            Instruction::Data(acc),
                        );
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
                        if cmp.unwrap() {
                            jump!(address)
                        }
                        cmp = None;
                    }
                    Operation::JumpNotEqual(address) => {
                        if !cmp.unwrap() {
                            jump!(address)
                        }
                        cmp = None;
                    }
                    Operation::Input => todo!(),
                    Operation::Output => println!("{}", acc as char),
                    Operation::End => break,
                }
            }
            pc += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exec() {
        let mut a = Assembler::from_csv(include_str!("9618_s21_qp_11.csv")).unwrap();
        println!("{:#?}", a);
        a.execute(200);
    }
}
