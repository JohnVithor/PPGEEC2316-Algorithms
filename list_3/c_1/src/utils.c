#include <stdio.h>
#include <stdlib.h>
#include "binary_heap.h"
#include "utils.h"

int* create_random_array(int n, int seed) {
    srand(seed);
    int* arr = (int*) malloc(n * sizeof(int)); 
    for (int i = 0; i < n; ++i){
        arr[i] = rand() % n;
    }
    return arr;
}

void print_array(Data* arr, int n) {
    for (int i = 0; i < n; ++i) {
        printf("key: %d - name: %s\n", arr[i].key, arr[i].name);
    }
    printf("\n");
}

void print_heap(BinaryHeap* heap) {
    int i = 0;
    int j = 0;
    int ident = 0;
    int* aux = (int*) malloc(heap->size * sizeof(int));
    while (i < heap->size) {
        printf("%*s", ident, "");
        printf("key: %d - name: %s\n", heap->data[i].key, heap->data[i].name);
        i++;
        if (i == 1 || i == 3 || i == 7 || i == 15 || i == 31 || i == 63) {
            ident += 2;
        }
    }
    free(aux);
}