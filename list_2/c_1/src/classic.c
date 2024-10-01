#include <stdio.h>
#include <stdlib.h>
#include <time.h>

void multiply(int* a, int* b, int* c, unsigned int n) {
    for(unsigned int i = 0; i < n; ++i) {
        for(unsigned int j = 0; j < n; ++j) {
            c[i * n + j] = 0;
            for(unsigned int k = 0; k < n; ++k) {
                c[i * n + j] += a[i * n + k] * b[k * n + j];
            }
        }
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

    srand(seed);
    int* a = (int*) malloc(n*n * sizeof(int)); 
    int* b = (int*) malloc(n*n * sizeof(int)); 
    int* c = (int*) malloc(n*n * sizeof(int)); 
    for (int i = 0; i < n*n; i++){
        a[i] = rand() % (n*n);
        b[i] = rand() % (n*n);
    }
    
    clock_t start_time = clock();
    multiply(a, b, c, n);
    clock_t end_time = clock();
    double time_taken = (double)(end_time - start_time) / CLOCKS_PER_SEC;
    printf("%lf\n", time_taken);

    for (int i = 0; i < n*n; i++){
        printf("%d ", a[i]);
    }
    printf("\n");
    for (int i = 0; i < n*n; i++){
        printf("%d ", b[i]);
    }
    printf("\n");
    for (int i = 0; i < n*n; i++){
        printf("%d ", c[i]);
    }
    printf("\n");

    free(a);
    free(b);
    free(c);
    return 0;
}
