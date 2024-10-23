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
    return randomized_select_kth(arr + q + 1, size - q - 1, i - k);
  }
}

size_t partition_around(int* arr, size_t p, size_t r, size_t x) {
    size_t i = p;
    size_t j = r;

    while (i <= j) {
        while (i <= r && arr[i] < x) {
            ++i;
        }
        while (j >= p && arr[j] > x) {
            --j;
        }
        if (i <= j) {
            if (i != j) {
                int tmp = arr[i];
                arr[i] = arr[j];
                arr[j] = tmp;
            }
            ++i;
            --j;
        }
    }
    return j;
}

int select_kth_internal(int* arr, size_t p, size_t r, size_t i) {
  if (r - p + 1 < 5) {
    insertion_sort(arr + p, r - p + 1);
    return arr[p + i - 1];
  }
  size_t g = (r - p + 1) / 5;
  for(size_t j = 0; j < g; ++j) {
    insertion_sort(arr + p + j * 5, 5);
  }
  int x = select_kth_internal(arr, p+2*g, p+3*g-1, g/2);
  size_t q = partition_around(arr, p, r, x);
  size_t k = q - p + 1;
  if (i == k) {
    return arr[q];
  } else if (i < k) {
    return select_kth_internal(arr, p, q, i);
  } else {
    return select_kth_internal(arr, q + 1, r, i - k);
  }
}

int select_kth(int* arr, size_t size, size_t i) {
  return select_kth_internal(arr, 0, size - 1, i);
}

