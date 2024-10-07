#include <stdlib.h>
#include "sort.h"
#include "utils.h"

void count_sort(int* arr, unsigned int size) {
    int k = array_max_value(arr, size) + 1;
    int* counter = (int*) malloc(k*sizeof(int));
    for (int i = 0; i < k; ++i) {
        counter[i] = 0;
    }
    for (unsigned int i = 0; i < size; ++i) {
        counter[arr[i]]++;
    }
    int i = 0;
    int j = 0;
    while (j < k) {
        if (counter[j] != 0) {
            arr[i++] = j;
            --counter[j];
        } else {
            ++j;
        }
    }
    free(counter);
}

unsigned long long count_sort_book_ram(int* arr, unsigned int size) {
    return 0;
}

unsigned long long count_sort_ram(int* arr, unsigned int size) {
    return 0;
}