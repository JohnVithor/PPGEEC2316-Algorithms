#include "matrix.h"

void matrix_add(int* a, int* b, int* c, unsigned int n) {
    for(unsigned int i = 0; i < n; ++i) {
        for(unsigned int j = 0; j < n; ++j) {
            c[i * n + j] = a[i * n + j] + b[i * n + j];
        }
    }
}

void matrix_subtract(int* a, int* b, int* c, unsigned int n) {
    for(unsigned int i = 0; i < n; ++i) {
        for(unsigned int j = 0; j < n; ++j) {
            c[i * n + j] = a[i * n + j] - b[i * n + j];
        }
    }
}

void matrix_multiply(int* a, int* b, int* c, unsigned int n) {
    for(unsigned int i = 0; i < n; ++i) {
        for(unsigned int j = 0; j < n; ++j) {
            c[i * n + j] = 0;
            for(unsigned int k = 0; k < n; ++k) {
                c[i * n + j] += a[i * n + k] * b[k * n + j];
            }
        }
    }
}
