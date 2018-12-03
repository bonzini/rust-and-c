#include <stdio.h>
#include "foo.h"

int foo_get(const Foo *foo)
{
	return foo->x;
}

void foo_set(Foo *foo, int value)
{
	foo->x = value;
}

void foo_print(const Foo *foo)
{
	printf("accessing from C: %d\n", foo->x);
}
