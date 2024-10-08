#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include "utils.h"

void merge(int* left, unsigned int left_size, int* right, unsigned int right_size, int* ret) {
    unsigned int l = 0;
    unsigned int r = 0;
    unsigned int i = 0;

    while(l < left_size && r < right_size) {
        ret[i++] = (left[l] < right[r]) ? left[l++] : right[r++];
    }

    if(l < left_size) {
        while (l < left_size){
            ret[i++] = left[l++];
        }
    }
    else {
        while (r < right_size){
            ret[i++] = right[r++];
        }
    }
}

void merge_sort(int* arr, unsigned int size) {
    unsigned int mid = size / 2;
    if (mid == 0) {
        return;
    }
    merge_sort(arr, mid);
    merge_sort(arr + mid, size - mid);

    int* aux = malloc(size*sizeof(int));
    for(unsigned int i = 0; i < size; ++i) {
        aux[i] = arr[i];
    }

    merge(aux, mid, aux + mid, size - mid, arr);
    free(aux);
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
    merge_sort(arr, n);
    clock_gettime(CLOCK_MONOTONIC, &ts_end);

    double time_taken_worse = (double)(ts_end.tv_sec - ts_start.tv_sec) + ((double)(ts_end.tv_nsec - ts_start.tv_nsec)/1000000000L);

    clock_gettime(CLOCK_MONOTONIC, &ts_start);
    merge_sort(arr, n);
    clock_gettime(CLOCK_MONOTONIC, &ts_end);
    double time_taken_best = (double)(ts_end.tv_sec - ts_start.tv_sec) + ((double)(ts_end.tv_nsec - ts_start.tv_nsec)/1000000000L);

    fill_random(arr, n, seed);
    
    clock_gettime(CLOCK_MONOTONIC, &ts_start);
    merge_sort(arr, n);
    clock_gettime(CLOCK_MONOTONIC, &ts_end);
    double time_taken_avg = (double)(ts_end.tv_sec - ts_start.tv_sec) + ((double)(ts_end.tv_nsec - ts_start.tv_nsec)/1000000000L);

    printf("%lf,%lf,%lf\n", time_taken_worse, time_taken_best, time_taken_avg);

    free(arr);
    return 0;
}
