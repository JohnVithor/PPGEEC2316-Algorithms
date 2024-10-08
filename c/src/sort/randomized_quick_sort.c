#include <stdlib.h>

#include "sort.h"

size_t randomized_partition(int* arr, size_t size) {
  size_t last_pos = size - 1;
  size_t random_pos = rand() % size;
  int pivot = arr[random_pos];
  arr[random_pos] = arr[last_pos];
  arr[last_pos] = pivot;
  size_t pivot_position = 0;
  int aux;
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

void randomized_quick_sort(int* arr, size_t size) {
  if (size > 1) {
    size_t pivot_position = randomized_partition(arr, size);
    randomized_quick_sort(arr, pivot_position);
    randomized_quick_sort(arr + pivot_position + 1, size - pivot_position - 1);
  }
}

size_t randomized_partition_ram(int* arr, size_t size,
                                      unsigned long long* op) {
  return 0;
}

unsigned long long randomized_quick_sort_ram(int* arr, size_t size) {
  return 0;
}