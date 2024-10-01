#include <stdio.h>
#include <stdlib.h>
#include <time.h>

unsigned long long  allocation(int* tasks, unsigned int n, int* workers, unsigned int k) {
    unsigned long long max_alloc = 0;
    unsigned int j = 0;
    for (unsigned int i = 0; i < k; ++i) {
        workers[i] = 0;
        for(j = i*(n/k); j < (i+1)*(n/k); ++j) {
            workers[i] += tasks[j];
        }
        if (workers[i] > max_alloc){
            max_alloc = workers[i];
        }
    }
    for (; j < n; ++j) {
        workers[k-1] += tasks[j];
    }
    if (workers[k-1] > max_alloc){
        max_alloc = workers[k-1];
    }
    return max_alloc;
}

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

    int* tasks   = (int*) malloc(n * sizeof(int)); 
    int* workers = (int*) malloc(k * sizeof(int)); 
    
    for (unsigned int i = 0; i < n; ++i){
        tasks[i] = i+1;
    }

    unsigned long long ascending = allocation(tasks, n, workers, k);

    for (unsigned int i = 0; i < n; ++i){
        if (i % 2 == 0)
            tasks[i] = i+1;    
        else
            tasks[i] = n-i+1;    
    }

    unsigned long long alternating = allocation(tasks, n, workers, k);

    srand(seed);

    for (unsigned int i = 0; i < n; ++i){
        unsigned int j = i + rand() % (n-i);
        int swap = tasks[i];
        tasks[i] = tasks[j];
        tasks[j] = swap;
    }

    unsigned long long random = allocation(tasks, n, workers, k);

    printf("%llu,%llu,%llu\n", ascending, alternating, random);

    free(tasks);
    free(workers);
    return 0;
}
