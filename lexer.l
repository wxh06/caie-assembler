%{
#include "y.tab.h"
%}

%% 

#[0-9]+ yylval = atoi(yytext + 1); return DENARY;

LDM return LDM;
OUT return OUT;

[ \t\n]+ ;

%%
