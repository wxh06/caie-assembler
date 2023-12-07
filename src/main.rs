use caie_assembler::parser::yyparse;

fn main() {
    unsafe {
        yyparse();
    }
}
