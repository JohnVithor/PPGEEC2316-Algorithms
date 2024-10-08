#ifndef _MATRIX_H_
#define _MATRIX_H_

#include <stddef.h>

void matrix_add(int* a, int* b, int* c, size_t n);
void matrix_subtract(int* a, int* b, int* c, size_t n);
void matrix_multiply(int* a, int* b, int* c, size_t n);
void matrix_multiply_strassen(int* a, int* b, int* c, size_t n);
void matrix_multiply_strassen_internal(int* a, int* b, int* c, size_t n,
                                       int* buffer);

#endif  // _MATRIX_H_
