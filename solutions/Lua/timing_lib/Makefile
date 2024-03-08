CC ?= clang
CFLAGS ?= -Wall -Wpedantic -Wconversion -O3 -march=native

TARGET_DIR ?= target

build: set_up_target
	$(CC) lua_timing.c -o $(TARGET_DIR)/timing.so $(CFLAGS) -shared -llua -fPIC

set_up_target:
	mkdir -p $(TARGET_DIR)
