#include "order_statistics.h"
#include "sort.h"
#include <stdio.h>


size_t partition_around(int* arr, size_t p, size_t r, size_t x) {
  int aux = arr[x];
  arr[x] = arr[r];
  arr[r] = aux;
  size_t i = p;
  for (size_t j = p; j < r; ++j) {
    if (arr[j] <= arr[r]) {
      aux = arr[i];
      arr[i++] = arr[j];
      arr[j] = aux;
    }
  }
  aux = arr[i];
  arr[i] = arr[r];
  arr[r] = aux;
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

int select_kth_internal(int* a, size_t p, size_t r, size_t i) {
  while ((r-p+1) % 5 != 0) {
    for (size_t j = p+1; j < r; ++j) {
      if (a[p] > a[j]) {
        int aux = a[j];
        a[j] = a[p];
        a[p] = aux;
      }
    }
    if (i == 1) {
      return a[p];
    }
    p = p + 1;
    i = i - 1;
  }
  size_t g = ((r-p+1) / 5);

  for(size_t j = p; j < p+g; ++j) {
    ref_sort_5(a + j, a + j+g, a + j+2*g, a + j+3*g, a + j+4*g);
  }

  size_t ceil_v = g%2 == 0 ? g/2 : g/2 + 1;
  size_t x = select_kth_internal(a, p+2*g, p+3*g-1, ceil_v);

  size_t q = partition_around(a, p, r, x);
  size_t k = q - p + 1;

  if (i == k) {
    return a[q];
  } else if (i < k) {
    return select_kth_internal(a, p, q-1, i);
  } else {
    return select_kth_internal(a, q + 1, r, i - k);
  }
}

int select_kth(int* arr, size_t size, size_t i) {
  return select_kth_internal(arr, 0, size-1, i);
}