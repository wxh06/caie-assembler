#include <stdio.h>

typedef int Number;
#define NUMBER Number number

void (*load_acc_number)(NUMBER);


void (*add_acc_number)(NUMBER);

void (*subtract_acc_number)(NUMBER);

void (*add_ix_number)(NUMBER);

void (*subtract_ix_number)(NUMBER);


void (*output)();

void (*end)();
