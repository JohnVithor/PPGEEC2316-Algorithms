#include <stdio.h>

#include "order_statistics.h"
#include "sort.h"

int randomized_select_kth(int* arr, size_t size, size_t i) {
  if (size == 1) {
    return 0;
  }
  size_t q = randomized_partition(arr, size);
  size_t k = q + 1;
  if (i == k) {
    return q;
  } else if (i < k) {
    return randomized_select_kth(arr, q, i);
  } else {
    return q + 1 + randomized_select_kth(arr + q + 1, size - q - 1, i - k);
  }
}

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
  size_t p = 0;
  while (size % 5 != 0) {
    for (size_t j = p+1; j < size; ++j) {
      if (arr[p] > arr[j]) {
        int aux = arr[j];
        arr[j] = arr[p];
        arr[p] = aux;
      }
    }
    if (i == 1) {
      return p;
    }
    p++;
    size--;
    i--;
  }

  size_t g = size / 5;

  // Ordena os grupos de 5
  for (size_t j = 0; j < g; ++j) {
    ref_sort_5(arr + p + j, arr + p + j + g, arr + p + j + 2 * g, arr + p + j + 3 * g,
               arr + p + j + 4 * g);
  }

  // Calcula a mediana das medianas
  size_t ceil_v = g % 2 == 0 ? g / 2 : g / 2 + 1;
  // Corrigido: usa o tamanho correto do subarray de medianas
  int x = select_kth(arr + p + 2 * g, g, ceil_v);

  size_t q = partition_around(arr + p, size, x);

  if (i == q + 1) {
    return p + q;
  } else if (i < q + 1) {
    return p + select_kth(arr + p, q, i);
  } else {
    return p + q + 1 + select_kth(arr + p + q + 1, size - (q + 1), i - (q + 1));
  }
}