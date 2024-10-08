#include "utils.h"

#include <stdio.h>
#include <stdlib.h>
#include <time.h>

int* create_random_array(size_t n, int seed) {
  srand(seed);
  int* arr = (int*)malloc(n * sizeof(int));
  for (size_t i = 0; i < n; ++i) {
    arr[i] = rand();
  }
  return arr;
}

void revert_array(int* arr, size_t n) {
  int aux;
  for (size_t i = 0; i < n / 2; ++i) {
    aux = arr[i];
    arr[i] = arr[n - i - 1];
    arr[n - i - 1] = aux;
  }
}

char validate_sorting(int* arr, size_t n) {
  for (size_t i = 0; i < n - 1; i++) {
    if (arr[i] > arr[i + 1]) {
      return 1;
    }
  }
  return 0;
}

void print_array(int* arr, size_t n) {
  printf("Array: [");
  for (size_t i = 0; i < n - 1; ++i) {
    printf("%d, ", arr[i]);
  }
  printf("%d]\n", arr[n - 1]);
}

double measure_time_sort(int* arr, size_t n,
                         void (*sort)(int*, size_t)) {
  struct timespec ts_start;
  struct timespec ts_end;
  clock_gettime(CLOCK_MONOTONIC, &ts_start);
  sort(arr, n);
  clock_gettime(CLOCK_MONOTONIC, &ts_end);
  return (double)(ts_end.tv_sec - ts_start.tv_sec) +
         ((double)(ts_end.tv_nsec - ts_start.tv_nsec) / 1000000000L);
}

int array_max_value(int* arr, size_t size) {
  int max = arr[0];
  for (size_t i = 1; i < size; ++i) {
    if (arr[i] > max) {
      max = arr[i];
    }
  }
  return max;
}

void* safe_malloc(size_t n) {
  void *p = malloc(n);
  if (p == NULL) {
    fprintf(stderr, "Fatal: failed to allocate %zu bytes.\n", n);
    abort();
  }
  return p;
}