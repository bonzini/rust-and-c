#include "foo.h"
#include <stdio.h>

int main()
{
	Foo x;
	foo_set(&x, 5);
	foo_print(&x);
	printf("accessing from Rust: %d\n", foo_get(&x));
}
