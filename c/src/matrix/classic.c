#include "matrix.h"

void matrix_add(int* a, int* b, int* c, size_t n) {
  for (size_t i = 0; i < n; ++i) {
    for (size_t j = 0; j < n; ++j) {
      c[i * n + j] = a[i * n + j] + b[i * n + j];
    }
  }
}

void matrix_subtract(int* a, int* b, int* c, size_t n) {
  for (size_t i = 0; i < n; ++i) {
    for (size_t j = 0; j < n; ++j) {
      c[i * n + j] = a[i * n + j] - b[i * n + j];
    }
  }
}

void matrix_multiply(int* a, int* b, int* c, size_t n) {
  for (size_t i = 0; i < n; ++i) {
    for (size_t j = 0; j < n; ++j) {
      c[i * n + j] = 0;
      for (size_t k = 0; k < n; ++k) {
        c[i * n + j] += a[i * n + k] * b[k * n + j];
      }
    }
  }
}
