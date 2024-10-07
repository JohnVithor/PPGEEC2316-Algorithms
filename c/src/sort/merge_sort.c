#include <stdlib.h>
#include "sort.h"

void merge(int* left, unsigned int left_size, int* right, unsigned int right_size, int* ret) {
    unsigned int l = 0;
    unsigned int r = 0;
    unsigned int i = 0;
    while(l < left_size && r < right_size) {
        ret[i++] = (left[l] < right[r]) ? left[l++] : right[r++];
    }
    if(l < left_size) {
        while (l < left_size){
            ret[i++] = left[l++];
        }
    } else {
        while (r < right_size){
            ret[i++] = right[r++];
        }
    }
}

void merge_sort(int* arr, unsigned int size) {
    unsigned int mid = size / 2;
    if (mid == 0) {
        return;
    }
    merge_sort(arr, mid);
    merge_sort(arr + mid, size - mid);
    int* aux = malloc(size*sizeof(int));
    for(unsigned int i = 0; i < size; ++i) {
        aux[i] = arr[i];
    }
    merge(aux, mid, aux + mid, size - mid, arr);
    free(aux);
}


unsigned long long merge_ram(int* left, unsigned int left_size, int* right, unsigned int right_size, int* ret) {
    return 0;
}

unsigned long long merge_sort_ram(int* arr, unsigned int size) {
    return 0;
}