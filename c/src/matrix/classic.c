#include "matrix.h"

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
    for (size_t j = 0; j < a->size; ++j) {
      matrix_set(c, i, j, 0);
      for (size_t k = 0; k < a->size; ++k) {
        matrix_e_add(c, i, j, matrix_get(a, i, k) * matrix_get(b, k, j));
      }
    }
  }
}

void matrix_multiply_transposed(Matrix* a, Matrix* b, Matrix* c) {
  transpose(b);
  for (size_t i = 0; i < a->size; ++i) {
    for (size_t j = 0; j < a->size; ++j) {
      matrix_set(c, i, j, 0);
      for (size_t k = 0; k < a->size; ++k) {
        matrix_e_add(c, i, j, matrix_get(a, i, k) * matrix_get(b, j, k));
      }
    }
  }
  transpose(b);
}

