CFLAGS = -O2 -g -fPIE

PROGRAMS = main
STATICLIBS = libfoo.a

.PHONY: all
all: main

main: src/main.o $(STATICLIBS)
	$(CC) -o $@ $^

libfoo.a: src/foo.o
	$(AR) cru $@ $^

%.o: %.c
	$(CC) -o $@ -c $< $(CFLAGS)

src/main.o: src/main.c src/foo.h
src/foo.o: src/foo.c src/foo.h

clean:
	rm -f $(PROGRAMS) $(STATICLIBS)
	find . -name "*.o" | xargs rm -f --

check:
	true
