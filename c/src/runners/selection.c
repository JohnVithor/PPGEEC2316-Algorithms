#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include "order_statistics.h"
#include "utils.h"

int main(int argc, char* argv[]) {
if (argc != 4) {
    printf("Uso: %s <n> <seed> <i> (n > 1 e 1 <= i <= n)\n", argv[0]);
    return 1;
  }
  int n = atoi(argv[1]);
  int seed = atoi(argv[2]);
  size_t i = atoi(argv[3]);
  if (n <= 1 || seed < 0 || i < 1 || i > n) {
    printf("Uso: %s <n> <seed> <i> (n > 1 e 1 <= i <= n)\n", argv[0]);
    return 1;
  }

  srand(seed);
  int* arr = (int*)safe_malloc(n * sizeof(int));
  for (size_t i = 0; i < n; ++i) {
    arr[i] = i;
  }
  for (size_t i = 0; i < n; ++i) {
    size_t j = rand() % n;
    int tmp = arr[i];
    arr[i] = arr[j];
    arr[j] = tmp;
  }

  struct timespec ts_start;
  struct timespec ts_end;
  clock_gettime(CLOCK_MONOTONIC, &ts_start);
  int r1 = randomized_select_kth(arr, n, i);
  clock_gettime(CLOCK_MONOTONIC, &ts_end);
  double time_spent_rand =
      (double)(ts_end.tv_sec - ts_start.tv_sec) +
      ((double)(ts_end.tv_nsec - ts_start.tv_nsec) / 1000000000L);

  clock_gettime(CLOCK_MONOTONIC, &ts_start);
  int r2 = select_kth(arr, n, i);
  clock_gettime(CLOCK_MONOTONIC, &ts_end);
  double time_spent_5_med =
      (double)(ts_end.tv_sec - ts_start.tv_sec) +
      ((double)(ts_end.tv_nsec - ts_start.tv_nsec) / 1000000000L);

  printf("%d,%d,%lf,%lf\n", r1, r2, time_spent_rand, time_spent_5_med);

  return 0;
}