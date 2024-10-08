#include <stdlib.h>

#include "sort.h"

void merge(int* left, size_t left_size, int* right,
           size_t right_size, int* ret) {
  size_t l = 0;
  size_t r = 0;
  size_t i = 0;
  while (l < left_size && r < right_size) {
    ret[i++] = (left[l] < right[r]) ? left[l++] : right[r++];
  }
  if (l < left_size) {
    while (l < left_size) {
      ret[i++] = left[l++];
    }
  } else {
    while (r < right_size) {
      ret[i++] = right[r++];
    }
  }
}

void merge_sort(int* arr, size_t size) {
  size_t mid = size / 2;
  if (mid == 0) {
    return;
  }
  merge_sort(arr, mid);
  merge_sort(arr + mid, size - mid);
  int* aux = malloc(size * sizeof(int));
  for (size_t i = 0; i < size; ++i) {
    aux[i] = arr[i];
  }
  merge(aux, mid, aux + mid, size - mid, arr);
  free(aux);
}

unsigned long long merge_ram(int* left, size_t left_size, int* right,
                             size_t right_size, int* ret) {
  return 0;
}

unsigned long long merge_sort_ram(int* arr, size_t size) { return 0; }