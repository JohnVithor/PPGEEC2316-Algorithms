#include "sort.h"

size_t partition(int* arr, size_t size) {
  int aux = 0;
  size_t last_pos = size - 1;
  int pivot = arr[last_pos];
  size_t pivot_position = 0;
  for (size_t i = 0; i < size; ++i) {
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