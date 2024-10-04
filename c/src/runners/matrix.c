#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include "matrix.h"
#include "utils.h"

int main(int argc, char *argv[]) {
    if (argc != 4) {
        printf("Uso: %s <n> <seed> <algoritmo> (n = 2^x, para algum x >= 1 e seed >=0])\n", argv[0]);
        printf("algoritmo:\n0 - classic multiply\n1 - strassen multiply\n");
        return 1;
    }
    int n = atoi(argv[1]);
    int seed = atoi(argv[2]);
    int algoritmo = atoi(argv[3]);

    if (n < 2 || (n & (n - 1)) != 0 || seed < 0) {
        printf("Uso: %s <n> <seed> <algoritmo> (n = 2^x, para algum x >= 1 e seed >=0])\n", argv[0]);
        printf("algoritmo:\n0 - classic multiply\n1 - strassen multiply\n");
        return 1;
    }

    void (*ops[2])(int* a, int* b, int* c, unsigned int n) = {matrix_multiply, matrix_multiply_strassen};
    char* op_names[2] = {"classic", "strassen"};

    srand(seed);
    int* a = (int*) malloc(n*n * sizeof(int)); 
    int* b = (int*) malloc(n*n * sizeof(int)); 
    int* c = (int*) malloc(n*n * sizeof(int)); 
    for (int i = 0; i < n*n; i++){
        a[i] = rand();
        b[i] = rand();
        c[i] = 0;
    }

    struct timespec ts_start;
    struct timespec ts_end;
    clock_gettime(CLOCK_MONOTONIC, &ts_start);
    (ops[algoritmo])(a, b, c, n);
    clock_gettime(CLOCK_MONOTONIC, &ts_end);
    double time_spent = (double)(ts_end.tv_sec - ts_start.tv_sec) + ((double)(ts_end.tv_nsec - ts_start.tv_nsec)/1000000000L);

    printf("%s,%lf\n", op_names[algoritmo], time_spent);

    free(a);
    free(b);
    free(c);
    return 0;
}
