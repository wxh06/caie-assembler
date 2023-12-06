use caie_assembler::yyparse;

fn main() {
    unsafe {
        yyparse();
    }
}
