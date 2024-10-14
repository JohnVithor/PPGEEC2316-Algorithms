#include "matrix.h"

inline Matrix matrix_create(T* data, size_t size) {
  Matrix m = {data, size, size};
  return m;
}

inline T matrix_get(Matrix* m, size_t i, size_t j) {
  return m->data[i * m->stride + j];
}

inline void matrix_set(Matrix* m, size_t i, size_t j, T x) {
  m->data[i * m->stride + j] = x;
}

inline void matrix_e_add(Matrix* m, size_t i, size_t j, T x) {
  m->data[i * m->stride + j] += x;
}

inline Matrix matrix_submatrix(Matrix* m, size_t i, size_t j, size_t size) {
  Matrix subm = {m->data + i * m->stride + j, size, m->stride};
  return subm;
}

void matrix_add(Matrix* a, Matrix* b, Matrix* c) {
  for (size_t i = 0; i < a->size; ++i) {
    for (size_t j = 0; j < b->size; ++j) {
      matrix_set(c, i, j, matrix_get(a, i, j) + matrix_get(b, i, j));
    }
  }
}

void matrix_subtract(Matrix* a, Matrix* b, Matrix* c) {
  for (size_t i = 0; i < a->size; ++i) {
    for (size_t j = 0; j < b->size; ++j) {
      matrix_set(c, i, j, matrix_get(a, i, j) - matrix_get(b, i, j));
    }
  }
}

void matrix_multiply(Matrix* a, Matrix* b, Matrix* c) {
  for (size_t i = 0; i < a->size; ++i) {
    for (size_t j = 0; j < b->size; ++j) {
      matrix_set(c, i, j, 0);
      for (size_t k = 0; k < a->size; ++k) {
        matrix_e_add(c, i, j, matrix_get(a, i, k) * matrix_get(b, k, j));
      }
    }
  }
}
