#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include "utils.h"

unsigned int partition(int* arr, unsigned int size) {
    int aux = 0;
    unsigned int pivot = size-1;
    int x = arr[pivot];
    unsigned int slow = 0;
    for (unsigned int fast = 0; fast < size; ++fast) {
        if (arr[fast] < x) {
            aux = arr[slow];
            arr[slow] = arr[fast];
            arr[fast] = aux;
            ++slow;
        }
    }
    aux = arr[slow];
    arr[slow] = arr[pivot];
    arr[pivot] = aux;

    return slow;
}

void quick_sort(int* arr, unsigned int size) {
    if (size > 1) {
        unsigned int pivot = partition(arr, size);
        quick_sort(arr, pivot);
        quick_sort(arr+pivot+1, size-pivot-1);
    }

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
    quick_sort(arr, n);
    clock_gettime(CLOCK_MONOTONIC, &ts_end);

    double time_taken_worse = (double)(ts_end.tv_sec - ts_start.tv_sec) + ((double)(ts_end.tv_nsec - ts_start.tv_nsec)/1000000000L);

    clock_gettime(CLOCK_MONOTONIC, &ts_start);
    quick_sort(arr, n);
    clock_gettime(CLOCK_MONOTONIC, &ts_end);

    double time_taken_best = (double)(ts_end.tv_sec - ts_start.tv_sec) + ((double)(ts_end.tv_nsec - ts_start.tv_nsec)/1000000000L);

    fill_random(arr, n, seed);
    
    clock_gettime(CLOCK_MONOTONIC, &ts_start);
    quick_sort(arr, n);
    clock_gettime(CLOCK_MONOTONIC, &ts_end);

    double time_taken_avg = (double)(ts_end.tv_sec - ts_start.tv_sec) + ((double)(ts_end.tv_nsec - ts_start.tv_nsec)/1000000000L);

    printf("%lf,%lf,%lf\n", time_taken_worse, time_taken_best, time_taken_avg);

    free(arr);
    return 0;
}
