CC ?= clang


debug:
	$(CC) main.c -o d1p1_debug -Wall -Wpedantic -Wconversion -O0 -g -fsanitize=address

build:
	$(CC) main.c -o d1p1 -Wall -Wpedantic -O3 -march=native -static

clean:
	rm d1p1

rund: debug
	./d1p1_debug

run: build
	./d1p1
