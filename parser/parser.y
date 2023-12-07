%{
#include <stdio.h>
#include "parser.h"
%}

%{
extern int yylex(void);

int yywrap()
{
	return 1;
}

void yyerror(const char *s)
{
	puts(s);
}
%}

%token NUMBER ACC IX
%token LDM LDD LDI LDX LDR MOV STO ADD SUB INC DEC JMP CMP CMI JPE JPN IN OUT END

%%

instructions
	:
	| instructions instruction
	;

instruction
	: LDM NUMBER
		{ load_acc_number($2); }

	| ADD NUMBER
		{ add_acc_number($2); }
	| SUB NUMBER
		{ subtract_acc_number($2); }

	| INC ACC
		{ add_acc_number(1); }
	| DEC ACC
		{ subtract_acc_number(1); }
	| INC IX
		{ add_ix_number(1); }
	| DEC IX
		{ subtract_ix_number(1); }

	| OUT
		{ output(); }
	| END
		{ end(); }
	;

%%
