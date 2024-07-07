CC = gcc
CFLAGS += -g -std=c99 -Wall -Wextra -Werror -Wfloat-equal -Wconversion -Wparentheses -pedantic -Wunused-parameter -Wunused-variable -Wreturn-type -Wno-unused-function -Wredundant-decls -Wreturn-type -Wunused-value -Wswitch-default -Wuninitialized -Winit-self -O2
FJAGE_VER=229b37c063eec87870cf129dbf94f766a97d0cb0
FJAGE_DIR=fjage-$(FJAGE_VER)
BUILD = build
BUILD_API = $(BUILD)/api
CONTRIB_DIR = $(BUILD)/temp

SAMPLE_SRC := $(wildcard samples/*.c)
SAMPLES_BIN := $(patsubst samples/%.c, samples/%, $(SAMPLE_SRC))

all: libs

samples: $(SAMPLES_BIN)

test: libs test/test_unet

libs: libfjage.a libunet.a

clean:
	rm -rf c-api libunet.a *.o samples/*.o test/*.o test/test_unet $(SAMPLES_BIN)
	rm -rf $(BUILD)

libfjage.a:
	cd ../../ && cargo build
	cp ../../target/debug/libfjage_rs.a libfjage.a

libunet.a: libfjage.a unet.o
	$(AR) rc $@ $^

unet.o: unet.c unet.h fjage.h
	$(CC) $(CFLAGS) -c -o $@ $<

samples/%.o: samples/%.c unet.h fjage.h unet.o libfjage.a
	$(CC) $(CFLAGS) -c -o $@ $<

%.o: %.c unet.h fjage.h
	$(CC) $(CFLAGS) -c -o $@ $<

samples/%: samples/%.o unet_ext.o unet.o libfjage.a
	$(CC) -o $@ $< unet_ext.o unet.o libfjage.a -lm -lpthread

test/%: test/%.o unet_ext.o unet.o libfjage.a
	$(CC) -o $@ $< unet_ext.o unet.o libfjage.a -lm -lpthread

.PHONY: all samples test libs package clean