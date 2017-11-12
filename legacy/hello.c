#include <stdio.h>
#include "hello.h"

void hello() {
#ifdef SAY_HI
	printf ("%s", SAY_HI);
#endif

    printf("Hello from C!\n");
}

void place(const char* place) {
    printf("We are at %s!\n", place);
}