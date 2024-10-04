#include <stdlib.h>
#include "sort.h"

unsigned int randomized_partition(int* arr, unsigned int size) {
    unsigned int last_pos = size-1;
    unsigned int random_pos = rand() % size;
    int pivot = arr[random_pos];
    arr[random_pos] = arr[last_pos];
    arr[last_pos] = pivot;
    unsigned int pivot_position = 0;
    int aux;
    for (unsigned int i = 0; i < size; ++i) {
        if (arr[i] < pivot) {
            aux = arr[pivot_position];
            arr[pivot_position++] = arr[i];
            arr[i] = aux;
        }
    }
    aux = arr[pivot_position];
    arr[pivot_position] = arr[last_pos];
    arr[last_pos] = aux;
    return pivot_position;
}

void randomized_quick_sort(int* arr, unsigned int size) {
    if (size > 1) {
        unsigned int pivot_position = randomized_partition(arr, size);
        randomized_quick_sort(arr, pivot_position);
        randomized_quick_sort(arr+pivot_position+1, size-pivot_position-1);
    }
}