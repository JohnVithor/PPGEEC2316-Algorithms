#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include "utils.h"
#include "order_statistics.h"


int main(int argc, char* argv[]) {
  if (argc != 2) {
    printf("Uso: %s <n> (n > 1])\n", argv[0]);
    return 1;
  }
  size_t n = 0;
  if (0 == sscanf(argv[1], "%zu", &n)){
    printf("valor inválido para <n>: %s\n", argv[1]);
    return 1;
  }
  if (n <= 1) {
    printf("Uso: %s <n> (n > 1)\n", argv[0]);
    return 1;
  }

  struct timespec ts_start;
  struct timespec ts_end;

  clock_gettime(CLOCK_MONOTONIC, &ts_start);
  int* arr = (int*)safe_malloc(n * sizeof(int));
  for (size_t i = 0; i < n; ++i) {
    arr[i] = i;
  }
  clock_gettime(CLOCK_MONOTONIC, &ts_end);
  double time_spent_loading =
      (double)(ts_end.tv_sec - ts_start.tv_sec) +
      ((double)(ts_end.tv_nsec - ts_start.tv_nsec) / 1000000000L);

  clock_gettime(CLOCK_MONOTONIC, &ts_start);
  Pair r1 = minimum_maximum_naive(arr, n);
  clock_gettime(CLOCK_MONOTONIC, &ts_end);
  double time_spent_naive =
      (double)(ts_end.tv_sec - ts_start.tv_sec) +
      ((double)(ts_end.tv_nsec - ts_start.tv_nsec) / 1000000000L);

  clock_gettime(CLOCK_MONOTONIC, &ts_start);
  Pair r2 = minimum_maximum(arr, n);
  clock_gettime(CLOCK_MONOTONIC, &ts_end);
  double time_spent =
      (double)(ts_end.tv_sec - ts_start.tv_sec) +
      ((double)(ts_end.tv_nsec - ts_start.tv_nsec) / 1000000000L);

  if (r1.min != r2.min || r1.max != r2.max) {
    printf("Valores diferentes:\nmin: %d e %d\nmax: %d e %d\n ", r1.min, r2.min, r1.max, r2.max);
  }

  printf("%lf,%lf,%lf\n", time_spent_loading, time_spent_naive, time_spent);

  free(arr);
  return 0;
}
