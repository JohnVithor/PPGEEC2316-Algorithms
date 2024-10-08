#include "sort.h"

#include <stdio.h>
#include <stdlib.h>

#include "utils.h"

int main(int argc, char* argv[]) {
  if (argc != 4) {
    printf("Uso: %s <n> <seed> <algoritmo> (n > 1 e seed >=0])\n", argv[0]);
    printf(
        "algoritmo:\n0 - insertion sort\n1 - merge sort\n2 - quick sort\n3 - "
        "randomized quick sort\n4 - count sort\n5 - radix sort\n");
    return 1;
  }
  int n = atoi(argv[1]);
  int seed = atoi(argv[2]);
  int algoritmo = atoi(argv[3]);

  if (n <= 1 || seed < 0 || algoritmo < 0 || algoritmo > 5) {
    printf("Uso: %s <n> <seed> <algoritmo> (n > 1 e seed >=0)\n", argv[0]);
    printf(
        "algoritmo:\n0 - insertion sort\n1 - merge sort\n2 - quick sort\n3 - "
        "randomized quick sort\n4 - count sort\n5 - radix sort\n");
    return 1;
  }

  void (*sorts[6])(int*, size_t) = {insertion_sort, merge_sort,
                                          quick_sort,     randomized_quick_sort,
                                          count_sort,     radix_sort};
  char* sort_names[6] = {"insertion_sort",        "merge_sort", "quick_sort",
                         "randomized_quick_sort", "count_sort", "radix_sort"};

  int* arr = create_random_array(n, seed);
  double time_spent_random = measure_time_sort(arr, n, sorts[algoritmo]);
  if (validate_sorting(arr, n)) {
    printf("Erro ao reverter o array random\n");
    return 1;
  }
  double time_spent_best = measure_time_sort(arr, n, sorts[algoritmo]);
  if (validate_sorting(arr, n)) {
    printf("Erro ao reverter o array crescente\n");
    return 1;
  }
  revert_array(arr, n);
  double time_spent_worse = measure_time_sort(arr, n, sorts[algoritmo]);
  if (validate_sorting(arr, n)) {
    printf("Erro ao reverter o array decrescente\n");
    return 1;
  }
  printf("%s,%lf,%lf,%lf\n", sort_names[algoritmo], time_spent_random,
         time_spent_best, time_spent_worse);
  return 0;
}