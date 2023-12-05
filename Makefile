a.out : lex.yy.c y.tab.c
	gcc lex.yy.c y.tab.c

lex.yy.c : asm.l
	flex asm.l

y.tab.c y.tab.h : asm.y
	bison -yd asm.y

.PHONY : clean
clean :
	rm -f lex.yy.c y.tab.c y.tab.h a.out
