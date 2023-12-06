#include "y.tab.c"
#include "assembler.h"

int main()
{
    yyparse();
    return 0;
}
