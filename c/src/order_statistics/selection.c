#include "order_statistics.h"
#include "sort.h"
#include <stdio.h>

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

size_t partition_around(int* arr, size_t size, size_t x) {
  int aux = arr[x];
  arr[x] = arr[size - 1];
  arr[size - 1] = aux;
  size_t i = 0;
  for (size_t j = 0; j < size-1; ++j) {
    if (arr[j] <= arr[size - 1]) {
      aux = arr[i];
      arr[i++] = arr[j];
      arr[j] = aux;
    }
  }
  aux = arr[i];
  arr[i] = arr[size - 1];
  arr[size - 1] = aux;
  return i;
}

void ref_sort_5(int* a, int* b, int* c, int* d, int* e) {
  int aux;
  if (*a > *b) {
    aux = *a;
    *a = *b;
    *b = aux;
  }
  if (*c > *d) {
    aux = *c;
    *c = *d;
    *d = aux;
  }
  if (*a > *c) {
    aux = *a;
    *a = *c;
    *c = aux;
  }
  if (*b > *d) {
    aux = *b;
    *b = *d;
    *d = aux;
  }
  if (*b > *c) {
    aux = *b;
    *b = *c;
    *c = aux;
  }
}

int select_kth(int* arr, size_t size, size_t i) {
  if (size < 5) {
    insertion_sort(arr, size);
    return arr[i-1];
  }
  size_t g = (size / 5);


  for(size_t j = 0; j < g; ++j) {
    ref_sort_5(arr + j, arr + j+g, arr + j+2*g, arr + j+3*g, arr + j+4*g);
  }

  size_t x = select_kth(arr+2*g, g, g%2 == 0 ? g/2 : g/2+1);
  size_t q = partition_around(arr, size, x);
  size_t k = q + 1;

  if (i == k) {
    return arr[q];
  } else if (i < k) {
    return select_kth(arr, q, i);
  } else {
    return select_kth(arr + q + 1, size - q - 1, i - k);
  }
}

