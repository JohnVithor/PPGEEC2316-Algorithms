#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include "utils.h"

void radix_sort(int* arr, unsigned int size) {
    int k = max(arr, size);
    int* c = (int*) malloc(size*sizeof(int));

    for (int i = 0; i < size; ++i) {
        c[i] = 0;
    }

    for (int exp = 1; k/exp > 0; exp *= 10) {
        int count[10] = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0};

        for (unsigned int i = 0; i < size; ++i) {
            ++count[(arr[i] / exp) % 10];
        }
        for (int i = 1; i < 10; i++) {
			count[i] += count[i - 1];
		}
        for (int i = size-1; i >= 0; --i) {
            c[--count[(arr[i] / exp) % 10]] = arr[i];
        }
        for(unsigned int i = 0; i < size; ++i){
            arr[i] = c[i];
        }
    }

    free(c);
}

int main(int argc, char** argv) {
    if (argc != 3) {
        printf("Uso: %s <n> <seed> (n > 1 e seed >=0)\n", argv[0]);
        return 1;
    }
    int n = atoi(argv[1]);
    int seed = atoi(argv[2]);

    if (n <= 1 || seed < 0) {
        printf("Uso: %s <n> <seed> (n > 1 e seed >=0)\n", argv[0]);
        return 1;
    }

    struct timespec ts_start;
    struct timespec ts_end;

    int* arr = create_desc_array(n);

    clock_gettime(CLOCK_MONOTONIC, &ts_start);
    radix_sort(arr, n);
    clock_gettime(CLOCK_MONOTONIC, &ts_end);

    double time_taken_worse = (double)(ts_end.tv_sec - ts_start.tv_sec) + ((double)(ts_end.tv_nsec - ts_start.tv_nsec)/1000000000L);

    clock_gettime(CLOCK_MONOTONIC, &ts_start);
    radix_sort(arr, n);
    clock_gettime(CLOCK_MONOTONIC, &ts_end);

    double time_taken_best = (double)(ts_end.tv_sec - ts_start.tv_sec) + ((double)(ts_end.tv_nsec - ts_start.tv_nsec)/1000000000L);

    fill_random(arr, n, seed);
    
    clock_gettime(CLOCK_MONOTONIC, &ts_start);
    radix_sort(arr, n);
    clock_gettime(CLOCK_MONOTONIC, &ts_end);

    double time_taken_avg = (double)(ts_end.tv_sec - ts_start.tv_sec) + ((double)(ts_end.tv_nsec - ts_start.tv_nsec)/1000000000L);

    printf("%lf,%lf,%lf\n", time_taken_worse, time_taken_best, time_taken_avg);

    free(arr);
    return 0;
}
