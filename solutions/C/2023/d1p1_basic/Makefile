CC ?= clang

TARGET_DIR ?= target

build: set_up_target
	$(CC) main.c -o $(TARGET_DIR)/d1p1 -Wall -Wpedantic -O3 -march=native -static

set_up_target:
	mkdir -p $(TARGET_DIR)

debug: set_up_target
	$(CC) main.c -o $(TARGET_DIR)/d1p1_debug -Wall -Wpedantic -Wconversion -O0 -g -fsanitize=address

clean:
	rm $(TARGET_DIR)/d1p1

rund: debug
	./$(TARGET_DIR)/d1p1_debug

run: build
	./$(TARGET_DIR)/d1p1
