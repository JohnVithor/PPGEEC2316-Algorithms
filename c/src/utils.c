#include <stdlib.h>
#include <stdio.h>
#include <time.h>
#include "utils.h"

int* create_random_array(int n, int seed) {
    srand(seed);
    int* arr = (int*) malloc(n * sizeof(int)); 
    for (int i = 0; i < n; ++i){
        arr[i] = rand();
    }
    return arr;
}

void revert_array(int* arr, unsigned int n) {
    int aux;
    for (unsigned int i = 0; i < n/2; ++i) {
        aux = arr[i];
        arr[i] = arr[n-i-1];
        arr[n-i-1] = aux;
    }
}

char validate_sorting(int* arr, unsigned int n) {
    for (int i = 0; i < n - 1; i++) {
        if (arr[i] > arr[i + 1]) {
            return 1;
        }
    }
    return 0;
}

void print_array(int* arr, unsigned int n) {
    printf("Array: [");
    for (unsigned int i = 0; i < n-1; ++i) {
        printf("%d, ", arr[i]);
    }
    printf("%d]\n", arr[n-1]);
}

double measure_sort_time(int* arr, unsigned int n, void (*sort)(int*, unsigned int)) {
    struct timespec ts_start;
    struct timespec ts_end;
    clock_gettime(CLOCK_MONOTONIC, &ts_start);
    sort(arr, n);
    clock_gettime(CLOCK_MONOTONIC, &ts_end);
    return (double)(ts_end.tv_sec - ts_start.tv_sec) + ((double)(ts_end.tv_nsec - ts_start.tv_nsec)/1000000000L);
}

int array_max_value(int* arr, unsigned int size) {
    int max = arr[0];
    for (unsigned int i = 1; i < size; ++i) {
        if (arr[i] > max) {
            max = arr[i];
        }
    }
    return max;
}