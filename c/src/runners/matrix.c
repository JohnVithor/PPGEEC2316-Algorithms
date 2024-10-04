#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include "matrix.h"
#include "utils.h"

int main(int argc, char *argv[]) {
    if (argc != 3) {
        printf("Uso: %s <n> <seed> (n = 2^x, para algum x >= 1 e seed >=0])\n", argv[0]);
        return 1;
    }
    int n = atoi(argv[1]);
    int seed = atoi(argv[2]);

    if (n < 2 || (n & (n - 1)) != 0 || seed < 0) {
        printf("Uso: %s <n> <seed> (n = 2^x, para algum x >= 1 e seed >=0])\n", argv[0]);
        return 1;
    }

    int* a = (int*) malloc(n*n * sizeof(int)); 
    int* b = (int*) malloc(n*n * sizeof(int)); 
    int* c = (int*) malloc(n*n * sizeof(int)); 
    int* d = (int*) malloc(n*n * sizeof(int)); 
    srand(seed);
    for (int i = 0; i < n*n; i++){
        a[i] = rand();
        b[i] = rand();
        c[i] = 0;
        d[i] = 0;
    }

    struct timespec ts_start;
    struct timespec ts_end;
    clock_gettime(CLOCK_MONOTONIC, &ts_start);
    matrix_multiply(a, b, c, n);
    clock_gettime(CLOCK_MONOTONIC, &ts_end);
    double time_spent_classic = (double)(ts_end.tv_sec - ts_start.tv_sec) + ((double)(ts_end.tv_nsec - ts_start.tv_nsec)/1000000000L);

    clock_gettime(CLOCK_MONOTONIC, &ts_start);
    matrix_multiply_strassen(a, b, d, n);
    clock_gettime(CLOCK_MONOTONIC, &ts_end);
    double time_spent_strassen = (double)(ts_end.tv_sec - ts_start.tv_sec) + ((double)(ts_end.tv_nsec - ts_start.tv_nsec)/1000000000L);

    for(int i = 0; i < n*n; i++) {
        if (c[i] != d[i]) {
            printf("Erro: c[%d] = %d != %d = d[%d]\n", i, c[i], d[i], i);
            return 1;
        }
    }

    printf("%lf,%lf\n", time_spent_classic, time_spent_strassen);

    free(a);
    free(b);
    free(c);
    free(d);
    return 0;
}
