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

%token DENARY LDM OUT

%%

instructions
	:
	| instructions instruction
	;

instruction
	: LDM DENARY
		{ load_acc_number($2); }
	| OUT
		{ output(); }
	;

%%
