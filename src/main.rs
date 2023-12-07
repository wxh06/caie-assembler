use caie_assembler::instruction::{Execution, Instruction};
use caie_assembler::operand::{Number, Operand};
use caie_assembler::parser;
use caie_assembler::register::Register;
use once_cell::unsync::Lazy;
use std::sync::Mutex;

static mut EXECUTION: Lazy<Mutex<Execution>> = Lazy::new(|| Default::default());

unsafe extern "C" fn load_acc_number(number: Number) {
    EXECUTION
        .lock()
        .unwrap()
        .instructions
        .push(Instruction::Load(Register::ACC, Operand::Number(number)))
}
unsafe extern "C" fn out() {
    EXECUTION
        .lock()
        .unwrap()
        .instructions
        .push(Instruction::Output)
}

fn main() {
    unsafe {
        parser::load_acc_number = Some(load_acc_number);
        parser::output = Some(out);

        parser::yyparse();

        EXECUTION.lock().unwrap().execute();
    }
}
