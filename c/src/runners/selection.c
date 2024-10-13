#include <stdio.h>
#include "order_statistics.h"

int main() {
  int arr[] = {3, 2, 9, 0, 7, 5, 4, 8, 6, 1};
  size_t n = 10;
  size_t i = 5;

  size_t r = randomized_select_kth(arr, n, i);
  printf("The %zu-th smallest element is %zu\n", i, r);

  return 0;
}