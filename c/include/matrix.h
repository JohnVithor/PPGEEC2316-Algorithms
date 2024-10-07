#ifndef _MATRIX_H_
#define _MATRIX_H_

void matrix_add(int* a, int* b, int* c, unsigned int n);
void matrix_subtract(int* a, int* b, int* c, unsigned int n);
void matrix_multiply(int* a, int* b, int* c, unsigned int n);
void matrix_multiply_strassen(int* a, int* b, int* c, unsigned int n);
void matrix_multiply_strassen_internal(int* a, int* b, int* c, unsigned int n,
                                       int* buffer);

#endif  // _MATRIX_H_
