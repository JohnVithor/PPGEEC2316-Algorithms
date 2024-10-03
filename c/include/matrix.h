#ifndef _MATRIX_H_
#define _MATRIX_H_

void matrix_add(int* a, int* b, int* c, unsigned int n);
void matrix_subtract(int* a, int* b, int* c, unsigned int n);
void matrix_multiply(int* a, int* b, int* c, unsigned int n);
void matrix_multiply_strassen(int* a, int* b, int* c, unsigned int n);

#endif // _MATRIX_H_
