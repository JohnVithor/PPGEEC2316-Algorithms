#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include "utils.h"
#include "order_statistics.h"

#define SIZE_MAX 250000000

int main(int argc, char* argv[]) {
  if (argc != 2) {
    printf("Uso: %s <seed> (seed >=0)\n", argv[0]);
    return 1;
  }
  int seed = atoi(argv[1]);

  if (seed < 0) {
    printf("Uso: %s <seed> (seed >=0)\n", argv[0]);
    return 1;
  }
  srand(seed);

  struct timespec ts_start;
  struct timespec ts_end;

  int* arr = (int*)safe_malloc(SIZE_MAX * sizeof(int));

  for (size_t i = 0; i < SIZE_MAX; ++i) {
    arr[i] = i;
  }
  for (size_t i = 0; i < SIZE_MAX; ++i) {
    size_t j = rand() % SIZE_MAX;
    int tmp = arr[i];
    arr[i] = arr[j];
    arr[j] = tmp;
  }

  size_t sizes[] = {
    10,20,30,40,50,60,70,80,90,
    100,200,300,400,500,600,700,800,900,
    1000,2000,3000,4000,5000,6000,7000,8000,9000,10000,
    20000,30000,40000,50000,60000,70000,80000,90000,
    100000,200000,300000,400000,500000,600000,700000,800000,900000,
    1000000, 2000000,3000000,4000000,5000000,6000000,7000000,8000000,9000000,
    10000000,20000000,30000000,40000000,50000000,60000000,70000000,80000000,90000000,
    100000000,200000000, 250000000
  };
  printf("size,run,naive,optimized\n");
  for (size_t i = 0; i < 66; ++i) {
    for (size_t j = 0; j < 100; ++j) {
      clock_gettime(CLOCK_MONOTONIC, &ts_start);
      ResultPair r1 = minimum_maximum_naive(arr, sizes[i]);
      clock_gettime(CLOCK_MONOTONIC, &ts_end);
      double time_spent_naive =
          (double)(ts_end.tv_sec - ts_start.tv_sec) +
          ((double)(ts_end.tv_nsec - ts_start.tv_nsec) / 1000000000L);

      clock_gettime(CLOCK_MONOTONIC, &ts_start);
      ResultPair r2 = minimum_maximum(arr, sizes[i]);
      clock_gettime(CLOCK_MONOTONIC, &ts_end);
      double time_spent =
          (double)(ts_end.tv_sec - ts_start.tv_sec) +
          ((double)(ts_end.tv_nsec - ts_start.tv_nsec) / 1000000000L);

      if (r1.status != r2.status || r1.pair.min != r2.pair.min || r1.pair.max != r2.pair.max) {
        printf("Valores diferentes:\nmin: %d e %d\nmax: %d e %d\n ", r1.pair.min, r2.pair.min, r1.pair.max, r2.pair.max);
      }
      printf("%zu,%zu,%lf,%lf\n", sizes[i], j+1, time_spent_naive, time_spent);
    }
  }
  free(arr);
  return 0;
}
