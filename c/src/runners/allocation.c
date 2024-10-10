#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include "utils.h"
#include "misc.h"
#include "sort.h"


int main(int argc, char** argv) {
    if (argc != 4) {
        printf("Uso: %s <n> <k> <seed> (n > 1 e seed >=0)\n", argv[0]);
        return 1;
    }

    unsigned int n = atoi(argv[1]);
    unsigned int k = atoi(argv[2]);
    int seed = atoi(argv[3]);

    if (n <= 2 || k < 1 || k > n || seed < 0) {
        printf("Uso: %s <n> <k> <seed> (n > 2, 1 < k < n e seed >=0)\n", argv[0]);
        return 1;
    }

    int* tasks   = (int*) safe_malloc(n * sizeof(int)); 
    int* workers = (int*) safe_malloc(k * sizeof(int)); 
    
    srand(seed);
    for (unsigned int i = 0; i < n; ++i){
        tasks[i] = rand() % 1000;
    }

    unsigned long long random = allocation(tasks, n, workers, k);

    quick_sort(tasks, n);

    unsigned long long sorted = allocation(tasks, n, workers, k);

    for (unsigned int i = 0; i < n/2; ++i){
        if (i % 2 == 0){
            int aux = tasks[i];
            tasks[i] = tasks[n-i-1];
            tasks[n-i-1] = aux;
        }
    }

    unsigned long long alternated = allocation(tasks, n, workers, k);

    printf("%llu,%llu,%llu\n", random, sorted, alternated);

    free(tasks);
    free(workers);
    return 0;
}