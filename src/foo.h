#ifndef FOO_H
#define FOO_H

typedef struct Foo {
	int x;
} Foo;

int foo_get(const Foo *x);
void foo_set(Foo *x, int value);
void foo_print(const Foo *x);

#endif
