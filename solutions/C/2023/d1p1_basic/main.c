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

uint64_t timespec_to_nanos(struct timespec time) { return time.tv_sec * 1000000000 + time.tv_nsec; }

uint32_t bench(uint64_t *runtime, uint32_t (*fn_ptr)(Buffer), Buffer input) {
#define MIN_SAMPLES 100
#define SAMPLE                                  \
    for (uint32_t i = 0; i < per_sample; i++) { \
        result = fn_ptr(input);                 \
    }
    struct timespec start;
    clock_gettime(CLOCK_MONOTONIC, &start);

    uint32_t result = fn_ptr(input);

    struct timespec end;
    clock_gettime(CLOCK_MONOTONIC, &end);
    uint64_t oneshot = timespec_to_nanos(end) - timespec_to_nanos(start);

    double   runs       = (double)*runtime / (double)oneshot;
    uint32_t per_sample = (uint32_t)runs / MIN_SAMPLES;

    per_sample = per_sample >= 2 ? per_sample : 2;
    per_sample = per_sample <= 32 ? per_sample : 32;

    clock_gettime(CLOCK_MONOTONIC, &start);
    SAMPLE;
    clock_gettime(CLOCK_MONOTONIC, &end);
    uint64_t sample_time = timespec_to_nanos(end) - timespec_to_nanos(start);

    uint64_t samples = *runtime / sample_time;
    samples          = samples >= 1 ? samples : 1;

    *runtime = 0;
    for (uint64_t sample = 0; sample < samples; sample++) {
        clock_gettime(CLOCK_MONOTONIC, &start);
        SAMPLE;
        clock_gettime(CLOCK_MONOTONIC, &end);
        uint64_t sample_time = timespec_to_nanos(end) - timespec_to_nanos(start);
        *runtime += sample_time;
    }
    *runtime = *runtime / samples / per_sample;

    return result;
}

int main(void) {
    Buffer input = read_input();

    uint64_t runtime = 128000000;

    uint32_t result = bench(&runtime, solve, input);

    printf("%d\n%ld", result, runtime);

    free(input.ptr);
}
