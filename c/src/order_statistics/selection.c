#include "order_statistics.h"
#include "sort.h"

size_t partition_around(int* arr, size_t size, size_t x) {
  int aux = arr[x];
  arr[x] = arr[size - 1];
  arr[size - 1] = aux;
  size_t i = 0;
  for (size_t j = 0; j < size - 1; ++j) {
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
    int temp;
    if (*a > *b) { temp = *a; *a = *b; *b = temp; }
    if (*c > *d) { temp = *c; *c = *d; *d = temp; }
    if (*c > *e) { temp = *c; *c = *e; *e = temp; }
    if (*d > *e) { temp = *d; *d = *e; *e = temp; }
    if (*a > *c) { temp = *a; *a = *c; *c = temp; }
    if (*b > *d) { temp = *b; *b = *d; *d = temp; }
    if (*b > *c) { temp = *b; *b = *c; *c = temp; }
    if (*d > *e) { temp = *d; *d = *e; *e = temp; }
    if (*c > *d) { temp = *c; *c = *d; *d = temp; }
}

int select_kth(int* arr, size_t size, size_t i) {
  while (size % 5 != 0) {
    for (size_t j = 1; j < size; ++j) {
      if (arr[0] > arr[j]) {
        int aux = arr[j];
        arr[j] = arr[0];
        arr[0] = aux;
      }
    }
    if (i == 1) {
      return arr[0];
    }
    arr++;
    size--;
    i--;
  }

  size_t g = size / 5;

  for (size_t j = 0; j < g; ++j) {
    ref_sort_5(arr + j, arr + j + g, arr + j + 2 * g, arr + j + 3 * g,
               arr + j + 4 * g);
  }

  size_t ceil_v = g % 2 == 0 ? g / 2 : g / 2 + 1;
  int x = select_kth(arr + 2 * g, g, ceil_v);

  size_t x_pos = 0;
  for (size_t j = 0; j < size; j++) {
    if (arr[j] == x) {
      x_pos = j;
      break;
    }
  }

  size_t q = partition_around(arr, size, x_pos);

  if (i == q + 1) {
    return arr[q];
  } else if (i < q + 1) {
    return select_kth(arr, q, i);
  } else {
    return select_kth(arr + q + 1, size - (q + 1), i - (q + 1));
  }
}