#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <unistd.h>

#define DEFAULT_CAP 8192

typedef struct {
    uint8_t *ptr;
    size_t   len;
    size_t   cap;
} Buffer;

void grow(Buffer *buf) {
    buf->cap = buf->cap ? buf->cap * 2 : DEFAULT_CAP;
    buf->ptr = (uint8_t *)realloc(buf->ptr, buf->cap);
}

void buffer_cleanup(Buffer buf) { free(buf.ptr); }

Buffer read_input(void) {
    Buffer buf = {malloc(DEFAULT_CAP), 0, DEFAULT_CAP};

    while (1) {
        size_t count = (size_t)read(0, buf.ptr + buf.len, buf.cap - buf.len);
        if (count <= 0) {
            break;
        }
        buf.len += count;
        if (buf.cap - buf.len < 4096) {
            grow(&buf);
        }
    }
    return buf;
}

uint32_t solve(Buffer input) {
    uint32_t sum_first = 0, sum_last = 0;
start:
    while (input.len > 0) {
        uint32_t first = 0, last = 0;
        while (input.len > 0) {
            uint8_t cur = *input.ptr;
            input.ptr++;
            input.len--;
            if (cur == '\n') {
                goto start;
            }
            uint8_t digit = cur - '0';
            if (digit <= 9) {
                first = digit;
                last  = first;
                break;
            }
        }
        while (input.len > 0) {
            uint8_t cur = *input.ptr;
            input.ptr++;
            input.len--;
            if (cur == '\n') {
                break;
            }
            uint8_t digit = cur - '0';
            if (digit <= 9) {
                last = digit;
            }
        }
        sum_first += first;
        sum_last += last;
    }
    return sum_first * 10 + sum_last;
}

int main(void) {
    Buffer input = read_input();

    struct timespec time;
    clock_gettime(CLOCK_MONOTONIC, &time);
    struct timespec start = time;

    uint32_t result = solve(input);

    clock_gettime(CLOCK_MONOTONIC, &time);
    struct timespec end = time;

    uint64_t nanos = (uint64_t)((end.tv_sec - start.tv_sec) * 1000000000 + end.tv_nsec - start.tv_nsec);

    printf("%d\n%ld", result, nanos);

    free(input.ptr);
}
