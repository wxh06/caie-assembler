use caie_assembler::instruction::{Execution, Instruction};
use caie_assembler::operand::{Number, Operand};
use caie_assembler::parser;
use caie_assembler::register::Register;
use once_cell::unsync::Lazy;
use std::sync::Mutex;

static mut EXECUTION: Lazy<Mutex<Execution>> = Lazy::new(|| Default::default());

pub unsafe fn append_instruction(instruction: Instruction) {
    EXECUTION.lock().unwrap().instructions.push(instruction)
}

unsafe extern "C" fn load_acc_number(number: Number) {
    append_instruction(Instruction::Load(Register::ACC, Operand::Number(number)))
}

unsafe extern "C" fn add_acc_number(number: Number) {
    append_instruction(Instruction::Add(Register::ACC, Operand::Number(number)))
}

unsafe extern "C" fn subtract_acc_number(number: Number) {
    append_instruction(Instruction::Subtract(
        Register::ACC,
        Operand::Number(number),
    ))
}

unsafe extern "C" fn add_ix_number(number: Number) {
    append_instruction(Instruction::Add(Register::ACC, Operand::Number(number)))
}

unsafe extern "C" fn subtract_ix_number(number: Number) {
    append_instruction(Instruction::Subtract(
        Register::ACC,
        Operand::Number(number),
    ))
}

unsafe extern "C" fn out() {
    append_instruction(Instruction::Output)
}
unsafe extern "C" fn end() {
    append_instruction(Instruction::End)
}

fn main() {
    unsafe {
        parser::load_acc_number = Some(load_acc_number);
        parser::add_acc_number = Some(add_acc_number);
        parser::subtract_acc_number = Some(subtract_acc_number);
        parser::add_ix_number = Some(add_ix_number);
        parser::subtract_ix_number = Some(subtract_ix_number);
        parser::output = Some(out);
        parser::end = Some(end);

        parser::yyparse();

        EXECUTION.lock().unwrap().execute();
    }
}
