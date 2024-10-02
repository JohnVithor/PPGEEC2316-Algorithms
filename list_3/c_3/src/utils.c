#include <stdio.h>
#include <stdlib.h>
#include "utils.h"

int* create_desc_array(int n) {
    int* arr = (int*) malloc(n * sizeof(int)); 
    for (int i = 0; i < n; i++){
        arr[i] = n-i;
    }
    return arr;
}

int* fill_random(int* arr, int n, int seed) {
    srand(seed);
    for (int i = 0; i < n; i++){
        arr[i] = rand();
    }
    return arr;
}

void print_array(int* arr, int n) {
    for (int i = 0; i < n; i++) {
        printf("%d ", arr[i]);
    }
    printf("\n");
}

void validate(int* arr, int n) {
    for (int i = 0; i < n - 1; i++) {
        if (arr[i] > arr[i + 1]) {
            printf("Erro de ordenação: %d < %d\n", arr[i], arr[i + 1]);
            return;
        }
    }
    printf("Ordenação correta\n");
}

int max(int* arr, unsigned int size) {
    int max = arr[0];
    for (unsigned int i = 1; i < size; ++i) {
        if (arr[i] > max) {
            max = arr[i];
        }
    }
    return max;
}