CARGO_TARGET = debug
CFLAGS = -O2 -g -fPIE
CARGO_BUILD_OPTS =
CARGO_TEST_OPTS =

PROGRAMS = main
STATICLIBS = libfoo.a

CARGO_LIBS = foo
CARGO_SOURCES = src/lib.rs

CARGO_STATICLIBS = $(CARGO_LIBS:%=target/$(CARGO_TARGET)/lib%.a)

ifeq ($(CARGO_TARGET), release)
CARGO_BUILD_OPTS += --release
endif

.PHONY: all
all: main

main: src/main.o $(STATICLIBS) $(CARGO_STATICLIBS)
	$(CC) -o $@ $^

libfoo.a: src/foo.o
	$(AR) cru $@ $^

%.o: %.c
	$(CC) -o $@ -c $< $(CFLAGS)

src/main.o: src/main.c src/foo.h
src/foo.o: src/foo.c src/foo.h

.PHONY: $(CARGO_STATICLIBS) cargo-build-lib
$(CARGO_STATICLIBS): cargo-build-lib

cargo-build-lib: $(CARGO_SOURCES)
	cargo build --lib $(CARGO_BUILD_OPTS)

clean:
	rm -f $(PROGRAMS) $(STATICLIBS)
	find . -name "*.o" | xargs rm -f --
	cargo clean

check:
	cargo test $(CARGO_BUILD_OPTS) $(CARGO_TEST_OPTS)
