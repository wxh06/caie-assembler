%{
#include <stdio.h>
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

%{
int ACC = 0;
%}

%token DENARY LDM OUT

%%

instructions
	:
	| instructions instruction
	;

instruction
	: LDM DENARY
		{ ACC = $2; }
	| OUT
		{ printf("%d\n", ACC); }
	;
%%
