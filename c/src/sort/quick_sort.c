#include "sort.h"

size_t partition(int* arr, size_t size) {
  int aux = arr[size - 1];
  arr[size - 1] = arr[size/2];
  arr[size/2] = aux;
  size_t i = 0;
  for (size_t j = 0; j < size-1; ++j) {
    if (arr[j] <= arr[size - 1]) {
      aux = arr[i];
      arr[i++] = arr[j];
      arr[j] = aux;
    }
  }
  aux = arr[i];
  arr[i] = arr[size - 1];
  arr[size - 1] = aux;
  return i;
}

void quick_sort(int* arr, size_t size) {
  if (size > 1) {
    size_t pivot_position = partition(arr, size);
    quick_sort(arr, pivot_position);
    quick_sort(arr + pivot_position + 1, size - pivot_position - 1);
  }
}

size_t partition_ram(int* arr, size_t size,
                           unsigned long long* op) {
  return 0;
}

unsigned long long quick_sort_ram(int* arr, size_t size) { return 0; }