#include <stdio.h>
#include "foo.h"

void foo_print(const Foo *foo)
{
	printf("accessing from C: %d\n", foo->x);
}
