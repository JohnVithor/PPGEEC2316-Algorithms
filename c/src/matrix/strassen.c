#include <stdlib.h>
#include "matrix.h"

void matrix_multiply_strassen(int* a, int* b, int* c, unsigned int n) {
    int* buffer = malloc(4*n*n*sizeof(int));
    matrix_multiply_strassen_internal(a, b, c, n, buffer);
    free(buffer);
}

void matrix_multiply_strassen_internal(int* a, int* b, int* c, unsigned int n, int* buffer) {
    if (n == 1) {
        c[0] = a[0] * b[0];
        return;
    }

    unsigned int mid = n / 2;

    int* a11 = buffer;
    int* a12 = a11 + (mid*mid);
    int* a21 = a12 + (mid*mid);
    int* a22 = a21 + (mid*mid);
    int* b11 = a22 + (mid*mid);
    int* b12 = b11 + (mid*mid);
    int* b21 = b12 + (mid*mid);
    int* b22 = b21 + (mid*mid);

    int* m1 = b22 + (mid*mid);
    int* m2 = m1 + (mid*mid);

    int* aux1 = m2 + (mid*mid);
    int* aux2 = aux1 + (mid*mid);
    int* remaining_buffer = aux2 + (mid*mid);


    for (unsigned int i = 0; i < mid; ++i) {
        for (unsigned int j = 0; j < mid; ++j) {
            a11[i * mid + j] = a[i         * n + j      ];
            a12[i * mid + j] = a[i         * n + j + mid];
            a21[i * mid + j] = a[(i + mid) * n + j      ];
            a22[i * mid + j] = a[(i + mid) * n + j + mid];
            b11[i * mid + j] = b[i         * n + j      ];
            b12[i * mid + j] = b[i         * n + j + mid];
            b21[i * mid + j] = b[(i + mid) * n + j      ];
            b22[i * mid + j] = b[(i + mid) * n + j + mid];
            c[i         * n + j       ] = 0;
            c[i         * n + j + mid ] = 0;
            c[(i + mid) * n + j       ] = 0;
            c[(i + mid) * n + j + mid ] = 0;
        }
    }

    matrix_add(a11, a22, aux1, mid);
    matrix_add(b11, b22, aux2, mid);
    matrix_multiply_strassen_internal(aux1, aux2, m1, mid, remaining_buffer);
    
    matrix_subtract(a12, a22, aux1, mid);
    matrix_add(b21, b22, aux2, mid);
    matrix_multiply_strassen_internal(aux1, aux2, m2, mid, remaining_buffer);

    matrix_subtract(b21, b11, aux1, mid);
    matrix_multiply_strassen_internal(a22, aux1, b21, mid, remaining_buffer);

    matrix_add(a21, a22, aux1, mid);
    matrix_multiply_strassen_internal(aux1, b11, a22, mid, remaining_buffer);

    matrix_add(a11, a12, aux2, mid);
    matrix_multiply_strassen_internal(aux2, b22, a12, mid, remaining_buffer);

    matrix_subtract(b12, b22, aux2, mid);
    matrix_multiply_strassen_internal(a11, aux2, b22, mid, remaining_buffer);

    matrix_subtract(a21, a11, aux1, mid);
    matrix_add(b11, b12, aux2, mid);
    matrix_multiply_strassen_internal(aux1, aux2, a21, mid, remaining_buffer);

    for (unsigned int i = 0; i < mid; ++i) {
        for (unsigned int j = 0; j < mid; ++j) {
            c[i * n + j]                = m1[i * mid + j] + m2[i * mid + j] + b21[i * mid + j] - a12[i * mid + j];
            c[(i + mid) * n + j + mid]  = m1[i * mid + j] - a22[i * mid + j] + b22[i * mid + j] + a21[i * mid + j];
            c[(i + mid) * n + j]        = a22[i * mid + j] + b21[i * mid + j];
            c[i * n + j + mid]          = a12[i * mid + j] + b22[i * mid + j];
        }
    }
}