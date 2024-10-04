#include <stdlib.h>
#include "matrix.h"

void matrix_multiply_strassen(int* a, int* b, int* c, unsigned int n) {
    if (n == 1) {
        c[0] = a[0] * b[0];
        return;
    }

    unsigned int mid = n / 2;

    int* a11 = malloc(mid*mid*sizeof(int));
    int* a12 = malloc(mid*mid*sizeof(int));
    int* a21 = malloc(mid*mid*sizeof(int));
    int* a22 = malloc(mid*mid*sizeof(int));
    int* b11 = malloc(mid*mid*sizeof(int));
    int* b12 = malloc(mid*mid*sizeof(int));
    int* b21 = malloc(mid*mid*sizeof(int));
    int* b22 = malloc(mid*mid*sizeof(int));
    int* c11 = malloc(mid*mid*sizeof(int));
    int* c12 = malloc(mid*mid*sizeof(int));
    int* c21 = malloc(mid*mid*sizeof(int));
    int* c22 = malloc(mid*mid*sizeof(int));

    int* m1 = malloc(mid*mid*sizeof(int));
    int* m2 = malloc(mid*mid*sizeof(int));
    int* m3 = malloc(mid*mid*sizeof(int));
    int* m4 = malloc(mid*mid*sizeof(int));
    int* m5 = malloc(mid*mid*sizeof(int));
    int* m6 = malloc(mid*mid*sizeof(int));
    int* m7 = malloc(mid*mid*sizeof(int));

    int* aux1 = malloc(mid*mid*sizeof(int));
    int* aux2 = malloc(mid*mid*sizeof(int));

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
        }
    }

    matrix_add(a11, a22, aux1, mid);
    matrix_add(b11, b22, aux2, mid);
    matrix_multiply_strassen(aux1, aux2, m1, mid);
    
    matrix_add(a21, a22, aux1, mid);
    matrix_multiply_strassen(aux1, b11, m2, mid);

    matrix_subtract(b12, b22, aux2, mid);
    matrix_multiply_strassen(a11, aux2, m3, mid);

    matrix_subtract(b21, b11, aux1, mid);
    matrix_multiply_strassen(a22, aux1, m4, mid);

    matrix_add(a11, a12, aux2, mid);
    matrix_multiply_strassen(aux2, b22, m5, mid);

    matrix_subtract(a21, a11, aux1, mid);
    matrix_add(b11, b12, aux2, mid);
    matrix_multiply_strassen(aux1, aux2, m6, mid);

    matrix_subtract(a12, a22, aux1, mid);
    matrix_add(b21, b22, aux2, mid);
    matrix_multiply_strassen(aux1, aux2, m7, mid);

    for (unsigned int i = 0; i < mid; ++i) {
        for (unsigned int j = 0; j < mid; ++j) {
            c11[i * mid + j] = m1[i * mid + j] + m4[i * mid + j] - m5[i * mid + j] + m7[i * mid + j];
            c12[i * mid + j] = m3[i * mid + j] + m5[i * mid + j];
            c21[i * mid + j] = m2[i * mid + j] + m4[i * mid + j];
            c22[i * mid + j] = m1[i * mid + j] - m2[i * mid + j] + m3[i * mid + j] + m6[i * mid + j];
        }
    }

    for (unsigned int i = 0; i < mid; ++i) {
        for (unsigned int j = 0; j < mid; ++j) {
            c[i         * n + j       ] = c11[i * mid + j];
            c[i         * n + j + mid ] = c12[i * mid + j];
            c[(i + mid) * n + j       ] = c21[i * mid + j];
            c[(i + mid) * n + j + mid ] = c22[i * mid + j];
        }
    }

    free(a11);
    free(a12);
    free(a21);
    free(a22);
    free(b11);
    free(b12);
    free(b21);
    free(b22);
    free(c11);
    free(c12);
    free(c21);
    free(c22);

    free(m1);
    free(m2);
    free(m3);
    free(m4);
    free(m5);
    free(m6);
    free(m7);

    free(aux1);
    free(aux2);
}
