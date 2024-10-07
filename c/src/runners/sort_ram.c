#include <stdio.h>
#include <stdlib.h>

#include "sort.h"
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

  if (n <= 1 || seed < 0) {
    printf("Uso: %s <n> <seed> <algoritmo> (n > 1 e seed >=0)\n", argv[0]);
    printf(
        "algoritmo:\n0 - insertion sort\n1 - merge sort\n2 - quick sort\n3 - "
        "randomized quick sort\n4 - count sort\n5 - radix sort\n");
    return 1;
  }

  long long unsigned int (*sorts[6])(int*, unsigned int) = {
      insertion_sort_ram,        merge_sort_ram, quick_sort_ram,
      randomized_quick_sort_ram, count_sort_ram, radix_sort_ram};
  char* sort_names[6] = {"insertion_sort_ram", "merge_sort_ram",
                         "quick_sort_ram",     "randomized_quick_sort_ram",
                         "count_sort_ram",     "radix_sort_ram"};

  int* arr = create_random_array(n, seed);
  long long unsigned int ops_random = (sorts[algoritmo])(arr, n);
  long long unsigned int ops_best = (sorts[algoritmo])(arr, n);
  revert_array(arr, n);
  long long unsigned int ops_worse = (sorts[algoritmo])(arr, n);
  printf("%s,%llu,%llu,%llu\n", sort_names[algoritmo], ops_random, ops_best,
         ops_worse);
  return 0;
}