#include <stdlib.h>

#include "matrix.h"

void matrix_multiply_strassen_internal(Matrix* a, Matrix* b, Matrix* c, T* buffer) {
  if (a->size == 1) {
    matrix_set(c, 0, 0, matrix_get(a, 0, 0) * matrix_get(b, 0, 0));
    return;
  }

  size_t mid = a->size / 2;

  MatrixSplit a_split = split4(a);
  Matrix a11 = a_split.m11;
  Matrix a12 = a_split.m12;
  Matrix a21 = a_split.m21;
  Matrix a22 = a_split.m22;

  MatrixSplit b_split = split4(b);
  Matrix b11 = b_split.m11;
  Matrix b12 = b_split.m12;
  Matrix b21 = b_split.m21;
  Matrix b22 = b_split.m22;

  Matrix aux1 = matrix_create(buffer, mid);
  Matrix aux2 = matrix_create(aux1.data + (mid * mid), mid);
  Matrix p1 = matrix_create(aux2.data + (mid * mid), mid);
  Matrix p2 = matrix_create(p1.data + (mid * mid), mid);
  Matrix p3 = matrix_create(p2.data + (mid * mid), mid);
  Matrix p4 = matrix_create(p3.data + (mid * mid), mid);
  Matrix p5 = matrix_create(p4.data + (mid * mid), mid);
  Matrix p6 = matrix_create(p5.data + (mid * mid), mid);
  Matrix p7 = matrix_create(p6.data + (mid * mid), mid);
  T* remaining_buffer = p7.data + (mid * mid);

  matrix_add(&a11, &a22, &aux1);
  matrix_add(&b11, &b22, &aux2);
  matrix_multiply_strassen_internal(&aux1, &aux2, &p1, remaining_buffer);
  
  matrix_add(&a21, &a22, &aux1);
  matrix_multiply_strassen_internal(&aux1, &b11, &p2, remaining_buffer);

  matrix_subtract(&b12, &b22, &aux2);
  matrix_multiply_strassen_internal(&a11, &aux2, &p3, remaining_buffer);

  matrix_subtract(&b21, &b11, &aux1);
  matrix_multiply_strassen_internal(&a22, &aux1, &p4, remaining_buffer);

  matrix_add(&a11, &a12, &aux2);
  matrix_multiply_strassen_internal(&aux2, &b22, &p5, remaining_buffer);

  matrix_subtract(&a21, &a11, &aux1);
  matrix_add(&b11, &b12, &aux2);
  matrix_multiply_strassen_internal(&aux1, &aux2, &p6, remaining_buffer);

  matrix_subtract(&a12, &a22, &aux1);
  matrix_add(&b21, &b22, &aux2);
  matrix_multiply_strassen_internal(&aux1, &aux2, &p7, remaining_buffer);

  for (size_t i = 0; i < mid; ++i) {
    for (size_t j = 0; j < mid; ++j) {
      matrix_set(c, i, j, matrix_get(&p1, i, j) + matrix_get(&p4, i, j) - matrix_get(&p5, i, j) + matrix_get(&p7, i, j));
      matrix_set(c, i, j + mid, matrix_get(&p3, i, j) + matrix_get(&p5, i, j));
      matrix_set(c, i + mid, j, matrix_get(&p2, i, j) + matrix_get(&p4, i, j));
      matrix_set(c, i + mid, j + mid, matrix_get(&p1, i, j) - matrix_get(&p2, i, j) + matrix_get(&p3, i, j) + matrix_get(&p6, i, j));
    }
  }
}

void matrix_multiply_strassen(Matrix* a, Matrix* b, Matrix* c) {
  T* buffer = malloc(4 * a->size * a->size * sizeof(T));
  matrix_multiply_strassen_internal(a, b, c, buffer);
  free(buffer);
}