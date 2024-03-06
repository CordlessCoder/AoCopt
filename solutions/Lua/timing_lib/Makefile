CC ?= clang

TARGET_DIR ?= target

build: set_up_target
	$(CC) lua_timing.c -o $(TARGET_DIR)/timing.so -shared -Wall -Wpedantic -Wconversion -O3 -march=native -llua -fPIC

set_up_target:
	mkdir -p $(TARGET_DIR)
