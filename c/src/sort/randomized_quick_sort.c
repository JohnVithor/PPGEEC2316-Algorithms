#include <stdlib.h>

#include "sort.h"

size_t randomized_partition(int* arr, size_t size) {
  size_t random_pos = rand() % size;
  int aux = arr[random_pos];
  arr[random_pos] = arr[size - 1];
  arr[size - 1] = aux;
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