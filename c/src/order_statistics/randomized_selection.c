#include "order_statistics.h"
#include "sort.h"

int randomized_select_kth(int* arr, size_t size, size_t i) {
  if (size == 1) {
    return arr[0];
  }
  size_t q = randomized_partition(arr, size);
  size_t k = q + 1;
  if (i == k) {
    return arr[q];
  } else if (i < k) {
    return randomized_select_kth(arr, q, i);
  } else {
    return randomized_select_kth(arr + k, size - q - 1, i - k);
  }
}

size_t randomized_select_kth_pos(int* arr, size_t size, size_t i) {
  if (size == 1) {
    return 0;
  }
  size_t q = randomized_partition(arr, size);
  size_t k = q + 1;
  if (i == k) {
    return q;
  } else if (i < k) {
    return randomized_select_kth_pos(arr, q, i);
  } else {
    return k + randomized_select_kth_pos(arr + k, size - q - 1, i - k);
  }
}