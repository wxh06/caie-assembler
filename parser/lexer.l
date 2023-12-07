%{
#include <stdlib.h>
#include "y.tab.h"
%}

%% 

#[0-9]+    yylval = strtoul(yytext + 1, NULL, 10); return NUMBER;
B[01]+     yylval = strtoul(yytext + 1, NULL, 2);  return NUMBER;
&[0-9A-F]+ yylval = strtoul(yytext + 1, NULL, 16); return NUMBER;

ACC return ACC;
IX  return IX;

LDM return LDM;
LDD return LDD;
LDI return LDI;
LDX return LDX;
LDR return LDR;
MOV return MOV;
STO return STO;
ADD return ADD;
SUB return SUB;
INC return INC;
DEC return DEC;
JMP return JMP;
CMP return CMP;
CMI return CMI;
JPE return JPE;
JPN return JPN;
IN  return IN;
OUT return OUT;
END return END;

[ \t\n]+ ;

%%
