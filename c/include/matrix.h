#ifndef _MATRIX_H_
#define _MATRIX_H_

#include <stddef.h>
#include <stdio.h>

typedef double T;

typedef struct Matrix {
  T* data;
  size_t size;
  size_t stride;
} Matrix;

typedef struct MatrixSplit {
  Matrix m11;
  Matrix m12;
  Matrix m21;
  Matrix m22;
} MatrixSplit;


inline Matrix matrix_create(T* data, size_t size) {
  Matrix m = {data, size, size};
  return m;
};

inline T matrix_get(Matrix* m, size_t i, size_t j) {
  return m->data[i * m->stride + j];
};

inline void matrix_set(Matrix* m, size_t i, size_t j, T x) {
  m->data[i * m->stride + j] = x;
};

inline void matrix_e_add(Matrix* m, size_t i, size_t j, T x) {
  m->data[i * m->stride + j] += x;
};

inline void print_m(Matrix m) {
  for(size_t i = 0; i < m.size; ++i) {
    for(size_t j = 0; j < m.size; ++j) {
      printf("%.2lf ", matrix_get(&m, i, j));
    }
    printf("\n");
  }
}

inline MatrixSplit split4(Matrix* m) {
  size_t mid = m->size / 2;
  MatrixSplit split = {
    {m->data, mid, m->stride},
    {m->data + mid, mid, m->stride},
    {m->data + mid * m->stride, mid, m->stride},
    {m->data + mid * m->stride + mid, mid, m->stride},
  };

  return split;
};

inline void transpose(Matrix* m) {
  for (size_t i = 0; i < m->size; ++i) {
    for (size_t j = i + 1; j < m->size; ++j) {
      T x = matrix_get(m, i, j);
      matrix_set(m, i, j, matrix_get(m, j, i));
      matrix_set(m, j, i, x);
    }
  }
};

void matrix_add(Matrix* a, Matrix* b, Matrix* c);
void matrix_subtract(Matrix* a, Matrix* b, Matrix* c);
void matrix_multiply(Matrix* a, Matrix* b, Matrix* c);
void matrix_multiply_transposed(Matrix* a, Matrix* b, Matrix* c);
void matrix_multiply_strassen(Matrix* a, Matrix* b, Matrix* c);
void matrix_multiply_strassen_internal(Matrix* a, Matrix* b, Matrix* c, T* buffer);

#endif  // _MATRIX_H_
