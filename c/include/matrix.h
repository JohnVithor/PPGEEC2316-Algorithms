#ifndef _MATRIX_H_
#define _MATRIX_H_

#include <stddef.h>

typedef double T;

typedef struct Matrix {
  T* data;
  size_t size;
  size_t stride;
} Matrix;

inline Matrix matrix_create(T* data, size_t size);
inline T matrix_get(Matrix* m, size_t i, size_t j);
inline void matrix_set(Matrix* m, size_t i, size_t j, T x);
inline void matrix_e_add(Matrix* m, size_t i, size_t j, T x);
inline Matrix matrix_submatrix(Matrix* m, size_t i, size_t j, size_t size);

void matrix_add(Matrix* a, Matrix* b, Matrix* c);
void matrix_subtract(Matrix* a, Matrix* b, Matrix* c);
void matrix_multiply(Matrix* a, Matrix* b, Matrix* c);
void matrix_multiply_strassen(Matrix* a, Matrix* b, Matrix* c);
void matrix_multiply_strassen_internal(Matrix* a, Matrix* b, Matrix* c, T* buffer);

#endif  // _MATRIX_H_
