#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <string.h>
#include "order_statistics.h"
#include "utils.h"

#define SIZE_MAX 250000000


void measure_print(int* arr, int* arr_backup, size_t size, size_t i, char* name) {
  struct timespec ts_start;
  struct timespec ts_end;
  for (size_t j = 1; j < 11; ++j) {
    clock_gettime(CLOCK_MONOTONIC, &ts_start);
    int r1 = randomized_select_kth(arr, size, i);
    clock_gettime(CLOCK_MONOTONIC, &ts_end);
    memcpy(arr, arr_backup, size * sizeof(int));
    double time_spent_rand =
        (double)(ts_end.tv_sec - ts_start.tv_sec) +
        ((double)(ts_end.tv_nsec - ts_start.tv_nsec) / 1000000000L);

    clock_gettime(CLOCK_MONOTONIC, &ts_start);
    int r2 = select_kth(arr, size, i);
    clock_gettime(CLOCK_MONOTONIC, &ts_end);
    memcpy(arr, arr_backup, size * sizeof(int));
    double time_spent_med =
        (double)(ts_end.tv_sec - ts_start.tv_sec) +
        ((double)(ts_end.tv_nsec - ts_start.tv_nsec) / 1000000000L);

    if (r1 != r2) {
      printf("Valores diferentes: %d e %d\n ", r1, r2);
    }
    printf("%zu,%s,%zu,%lf,%lf\n", size, name, j, time_spent_rand, time_spent_med);
  }
}

int main(int argc, char* argv[]) {
  if (argc != 2) {
    printf("Uso: %s <file> \n", argv[0]);
    return 1;
  }
  FILE* f = fopen(argv[1], "rb");
  if (f == NULL) {
    printf("Erro ao abrir o arquivo\n");
    return 1;
  }

  int* arr = (int*)safe_malloc(SIZE_MAX * sizeof(int));
  int* arr_backup = (int*)safe_malloc(SIZE_MAX * sizeof(int));
  fread(arr, sizeof(int), SIZE_MAX, f);
  memcpy(arr_backup, arr, SIZE_MAX * sizeof(int));
  fclose(f);

  size_t sizes[] = {
    10,20,30,40,50,60,70,80,90,
    100,200,300,400,500,600,700,800,900,
    1000,2000,3000,4000,5000,6000,7000,8000,9000,
    10000,20000,30000,40000,50000,60000,70000,80000,90000,
    100000,200000,300000,400000,500000,600000,700000,800000,900000,
    1000000,2000000,3000000,4000000,5000000,6000000,7000000,8000000,9000000,
    10000000,20000000,30000000,40000000,50000000,60000000,70000000,80000000,90000000,
    100000000,200000000,250000000
  };

  printf("size,i,run,randomized,median_of_medians\n");
  for (size_t i = 0; i < 66; ++i) {
    measure_print(arr, arr_backup, sizes[i], 1, "min");
    measure_print(arr, arr_backup, sizes[i], sizes[i]/10, "1/10");
    measure_print(arr, arr_backup, sizes[i], sizes[i]/4, "1/4");
    measure_print(arr, arr_backup, sizes[i], sizes[i]/3, "1/3");
    measure_print(arr, arr_backup, sizes[i], sizes[i]/2, "median");
    measure_print(arr, arr_backup, sizes[i], 2*sizes[i]/3, "2/3");
    measure_print(arr, arr_backup, sizes[i], 3*sizes[i]/4, "3/4");
    measure_print(arr, arr_backup, sizes[i], 9*sizes[i]/10, "9/10");
    measure_print(arr, arr_backup, sizes[i], sizes[i], "max");
  }
  free(arr);
  return 0;
}
