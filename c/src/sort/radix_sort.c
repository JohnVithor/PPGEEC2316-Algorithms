#include <stdlib.h>
#include "sort.h"
#include "utils.h"

void radix_sort(int* arr, unsigned int size) {
    int k = array_max_value(arr, size);
    int* c = (int*) malloc(size*sizeof(int));
    for (int i = 0; i < size; ++i) {
        c[i] = 0;
    }
    for (int exp = 1; k/exp > 0; exp *= 10) {
        int count[10] = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0};
        for (unsigned int i = 0; i < size; ++i) {
            ++count[(arr[i] / exp) % 10];
        }
        for (int i = 1; i < 10; i++) {
			count[i] += count[i - 1];
		}
        for (int i = size-1; i >= 0; --i) {
            c[--count[(arr[i] / exp) % 10]] = arr[i];
        }
        for(unsigned int i = 0; i < size; ++i){
            arr[i] = c[i];
        }
    }
    free(c);
}

unsigned long long radix_sort_ram(int* arr, unsigned int size) {
    return 0;
}