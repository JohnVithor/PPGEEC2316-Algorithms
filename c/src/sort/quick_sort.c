#include "sort.h"

unsigned int partition(int* arr, unsigned int size) {
  int aux = 0;
  unsigned int last_pos = size - 1;
  int pivot = arr[last_pos];
  unsigned int pivot_position = 0;
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

void quick_sort(int* arr, unsigned int size) {
  if (size > 1) {
    unsigned int pivot_position = partition(arr, size);
    quick_sort(arr, pivot_position);
    quick_sort(arr + pivot_position + 1, size - pivot_position - 1);
  }
}

unsigned int partition_ram(int* arr, unsigned int size,
                           unsigned long long* op) {
  return 0;
}

unsigned long long quick_sort_ram(int* arr, unsigned int size) { return 0; }