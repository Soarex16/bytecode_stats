#include "stdio.h"
#include "runtime.c"

int main() {
   __gc_init();
   void* p = Barray(BOX(3), BOX(1), BOX(2), BOX(3));
   int first = UNBOX(Belem(p, BOX(1)));
   printf("allocated array %p", p);
   return 0;
}