#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#include "sort.h"
#include "utils.h"

int TASK_SIZE = 2000;

void merge_sort_parallel(int* arr, size_t size) {
  size_t mid = size / 2;
  if (mid == 0) {
    return;
  }
  // #pragma omp task shared(arr) if (size > TASK_SIZE)
  // merge_sort_parallel(arr, mid);
  // merge_sort_parallel(arr + mid, size - mid);
  // int* aux = malloc(size * sizeof(int));
  // #pragma omp taskwait

  #pragma omp parallel sections
  {
      #pragma omp section
      {
          merge_sort_parallel(arr, mid);
      }
      #pragma omp section
      {
          merge_sort_parallel(arr + mid, size - mid);
      }
  }

  int* aux = malloc(size * sizeof(int));
  for (size_t i = 0; i < size; ++i) {
    aux[i] = arr[i];
  }
  merge(aux, mid, aux + mid, size - mid, arr);
  free(aux);
}

double measure_time(int* arr, size_t n, void (*sort)(int*, size_t)) {
  struct timespec ts_start;
  struct timespec ts_end;
  clock_gettime(CLOCK_MONOTONIC, &ts_start);
  #pragma omp parallel
  {
    #pragma omp single
    sort(arr, n);
  }
  
  clock_gettime(CLOCK_MONOTONIC, &ts_end);
  return (double)(ts_end.tv_sec - ts_start.tv_sec) +
         ((double)(ts_end.tv_nsec - ts_start.tv_nsec) / 1000000000L);
}

int main(int argc, char* argv[]) {
  if (argc != 4) {
    printf("Uso: %s <n> <seed> <MAX_VALUE> (n > 1 e seed >=0])\n", argv[0]);
    return 1;
  }
  int n = atoi(argv[1]);
  int seed = atoi(argv[2]);
  int max_value =  atoi(argv[3]);
  if (max_value <= 0) {
    max_value = RAND_MAX;
  }
  if (n <= 1 || seed < 0) {
    printf("Uso: %s <n> <seed> <MAX_VALUE> (n > 1 e seed >=0)\n", argv[0]);
    return 1;
  }


  int* arr = create_random_array(n, seed, max_value);
  int* backup = malloc(n * sizeof(int));
  for (size_t i = 0; i < n; ++i) {
    backup[i] = arr[i];
  }

  double time_spent_random = measure_time(arr, n, merge_sort);
  double time_spent_random_p = measure_time(backup, n, merge_sort_parallel);
  if (validate_sorting(arr, n)) {
    printf("Erro ao reverter o array random\n");
    return 1;
  }
  double time_spent_best = measure_time(arr, n, merge_sort);
  double time_spent_best_p = measure_time(backup, n, merge_sort_parallel);
  if (validate_sorting(arr, n)) {
    printf("Erro ao reverter o array crescente\n");
    return 1;
  }
  revert_array(arr, n);
  revert_array(backup, n);
  double time_spent_worse = measure_time(arr, n, merge_sort);
  double time_spent_worse_p = measure_time(backup, n, merge_sort_parallel);
  if (validate_sorting(arr, n)) {
    printf("Erro ao reverter o array decrescente\n");
    return 1;
  }
  printf("%s,%lf,%lf,%lf\n", "merge_sort", time_spent_random, time_spent_best, time_spent_worse);
  printf("%s,%lf,%lf,%lf\n", "merge_sort_parallel", time_spent_random_p, time_spent_best_p, time_spent_worse_p);
  return 0;
}