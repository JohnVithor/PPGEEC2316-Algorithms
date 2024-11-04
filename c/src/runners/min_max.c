#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include "utils.h"
#include "order_statistics.h"

#define SIZE_MAX 250000000

int main(int argc, char* argv[]) {
  if (argc != 2) {
    printf("Uso: %s <path>\n", argv[0]);
    return 1;
  }
  FILE* f = fopen(argv[1], "rb");
  if (f == NULL) {
    printf("Erro ao abrir o arquivo\n");
    return 1;
  }
  int* v = (int*)safe_malloc(SIZE_MAX * sizeof(int));
  fread(v, sizeof(int), SIZE_MAX, f);
  fclose(f);
  struct timespec ts_start;
  struct timespec ts_end;

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
  double* arr_time_spent_naive = (double*)safe_malloc(66 * 100 * sizeof(double));
  double* arr_time_spent_opt = (double*)safe_malloc(66 * 100 * sizeof(double));

  printf("size,run,naive,optimized\n");
  for (size_t i = 0; i < 66; ++i) {
    for (size_t j = 0; j < 100; ++j) {
      clock_gettime(CLOCK_MONOTONIC, &ts_start);
      ResultPair r1 = minimum_maximum_naive(v, sizes[i]);
      clock_gettime(CLOCK_MONOTONIC, &ts_end);
      double time_spent_naive =
          (double)(ts_end.tv_sec - ts_start.tv_sec) +
          ((double)(ts_end.tv_nsec - ts_start.tv_nsec) / 1000000000L);
      arr_time_spent_naive[i*100+j] = time_spent_naive;
    }
  }
  for (size_t i = 0; i < 66; ++i) {
    for (size_t j = 0; j < 100; ++j) {
      clock_gettime(CLOCK_MONOTONIC, &ts_start);
      ResultPair r2 = minimum_maximum(v, sizes[i]);
      clock_gettime(CLOCK_MONOTONIC, &ts_end);
      double time_spent_opt =
          (double)(ts_end.tv_sec - ts_start.tv_sec) +
          ((double)(ts_end.tv_nsec - ts_start.tv_nsec) / 1000000000L);
      arr_time_spent_opt[i*100+j] = time_spent_opt;
    }
  }
  for (size_t i = 0; i < 66; ++i) {
    for (size_t j = 0; j < 100; ++j) {
      printf("%zu,%zu,%lf,%lf\n", sizes[i], j+1, arr_time_spent_naive[i*100+j], arr_time_spent_opt[i*100+j]);
    }
  }
  free(v);
  free(arr_time_spent_naive);
  free(arr_time_spent_opt);
  return 0;
}
