pub type Address = usize;
pub type Number = i32;

pub enum Operand {
    Number(Number),
    Address(Address),
}
