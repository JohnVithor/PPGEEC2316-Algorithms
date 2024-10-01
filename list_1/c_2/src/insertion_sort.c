#include <stdio.h>
#include <stdlib.h>
#include <time.h>

void insertion_sort(int* arr, unsigned int n) {
    for (unsigned int i = 1; i < n; ++i) {
        int key = arr[i];
        unsigned int j = i;
        while (j > 0 && arr[j-1] > key) {
            arr[j] = arr[j-1];
            --j;
        }
        arr[j] = key;
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

    int* arr = (int*) malloc(n * sizeof(int)); 
    srand(seed);
    for (int i = 0; i < n; i++){
        arr[i] = rand() % n;
    }
    
    struct timespec ts_start;
    struct timespec ts_end;
    
    clock_gettime(CLOCK_MONOTONIC, &ts_start);
    insertion_sort(arr, n);
    clock_gettime(CLOCK_MONOTONIC, &ts_end);

    double time_taken = (double)(ts_end.tv_sec - ts_start.tv_sec) + ((double)(ts_end.tv_nsec - ts_start.tv_nsec)/1000000000L);
    printf("%lf\n", time_taken);

    free(arr);
    return 0;
}
